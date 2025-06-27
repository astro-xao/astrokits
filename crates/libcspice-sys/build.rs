use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const CSPICE_DIR: &str = "CSPICE_DIR";

fn main() {
    println!("cargo:rerun-if-env-changed={}", CSPICE_DIR);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cspice_dir = env::var(CSPICE_DIR).ok().map(PathBuf::from);

    let cspice_include = if cspice_dir.is_some() {
        cspice_dir.as_ref().unwrap().join("include")
    } else {
        PathBuf::from("vendor/cspice/include")
    };

    gen_bindings(&cspice_include);

    #[cfg(feature = "cspice-src")]
    let cspice_dir = cspice_dir.or_else(|| {
        let downloaded = out_path.join("cspice");
        if !downloaded.exists() {
            download_cspice(&out_path);
        }
        Some(out_path)
    });

    let cspice_dir = match cspice_dir {
        Some(dir) => {
            if !dir.exists() {
                println!("cargo:warning={}", format!("`cspice_dir` does not point to a valid directory: {}", dir.display()));
                return;
            }
            dir
        },
        None => {
            println!("cargo:warning={}", format!("`cspice_dir` does not point to a valid directory. Please set the {} environment variable or use `cspice-src` feature.", CSPICE_DIR));
            return;
        }
    };

    let mut cfg = cc::Build::new();

    #[cfg(feature = "cspice-src")]
    build_cspicelib(&mut cfg, &cspice_dir.join("cspice"));

    let cspice_lib = cspice_dir.join("lib");
    let cspice_include = cspice_dir.join("include");

    println!("cargo:rustc-link-search=native={}", cspice_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=cspice");
    println!("cargo:include={}", cspice_include.to_str().unwrap());
}

#[cfg(feature = "cspice-src")]
fn build_cspicelib(cfg: &mut cc::Build, cspice_dst: &PathBuf) {
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib = dst.join("lib");

    cfg.warnings(false).out_dir(&lib);

    println!("cargo:warning={}", cspice_dst.join("src/cspice").display());

    let src_files: Vec<_> = fs::read_dir(&cspice_dst.join("src/cspice"))
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("c") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    cfg.files(&src_files);

    let target = env::var("TARGET").unwrap();

    if target.contains("msvc") {
        cfg.flag_if_supported("/c")
            .flag_if_supported("/TC")
            .flag_if_supported("/MP")
            .flag_if_supported("/O2")
            .define("KR_headers", None)
            .define("_COMPLEX_DEFINED", None)
            .define("MSDOS", None)
            .define("OMIT_BLANK_CC", None)
            .define("NON_ANSI_STDIO", None);
    }

    if target.contains("gnu") {
        cfg.flag_if_supported("-c")
            .flag_if_supported("-ansi")
            .flag_if_supported("-m64")
            .flag_if_supported("-O2")
            .flag_if_supported("-fPIC")
            .define("NON_UNIX_STDIO", None);
    }

    cfg.compile("cspice");

    fs::create_dir_all(dst.join("include/cspice")).unwrap();
    fs::read_dir(cspice_dst.join("include"))
        .unwrap()
        .into_iter()
        .for_each(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("h") {
                let file_name = path.file_name().unwrap();
                fs::copy(entry.path(), dst.join("include/cspice").join(file_name)).unwrap();
            }
        });
}

fn gen_bindings(dst: &PathBuf) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Generate the bindings
    let bindings = bindgen::Builder::default()
        .header(dst.join("cspice/SpiceUsr.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
}

// Fetch CSPICE source from NAIF servers and extract to `<out_dir>/cspice`
#[cfg(feature = "cspice-src")]
fn download_cspice(out_dir: &PathBuf) {
    // Pick appropriate package to download
    let (platform, extension) = match env::consts::OS {
        "linux" => ("PC_Linux_GCC_64bit", "tar.Z"),
        "macos" => (
            if cfg!(target_arch = "arm") {
                "MacM1_OSX_clang_64bit"
            } else {
                "MacIntel_OSX_AppleC_64bit"
            },
            "tar.Z",
        ),
        "windows" => ("PC_Windows_VisualC_64bit", "zip"),
        _ => {
            unimplemented!("Cannot fetch CSPICE source for this platform, please download manually")
        }
    };

    let url = format!(
        "https://naif.jpl.nasa.gov/pub/naif/toolkit//C/{}/packages/cspice.{}",
        platform, extension
    );

    let download_target = out_dir.join(format!("cspice.{}", extension));

    let body = reqwest::blocking::get(url)
        .expect("Failed to download CSPICE")
        .bytes()
        .unwrap();
    std::fs::write(download_target, body).expect("Failed to write archive file");

    // Extract package based on platform
    match (env::consts::OS, extension) {
        ("linux" | "macos", "tar.Z") => {
            Command::new("gzip")
                .current_dir(out_dir)
                .args(["-d", "cspice.tar.Z"])
                .status()
                .expect("Failed to extract with gzip");
            Command::new("tar")
                .current_dir(out_dir)
                .args(["xf", "cspice.tar"])
                .status()
                .expect("Failed to extract with tar");

            fs::rename(
                out_dir.join("cspice/lib/cspice.a"),
                out_dir.join("cspice/lib/libcspice.a"),
            )
            .unwrap();
        }
        ("windows", "zip") => {
            Command::new("tar")
                .current_dir(out_dir)
                .args(["xf", "cspice.zip"])
                .status()
                .expect("Failed to extract with tar");
        }
        _ => unreachable!(),
    }
}
