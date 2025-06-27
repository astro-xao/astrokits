use std::path::PathBuf;
use std::{env, fs};
use std::process::Command;
use cc::Build;

const SUPERNOVAS_DIR: &str = "SUPERNOVAS_DIR";

fn main() {
    println!("cargo:rerun-if-env-changed={}", SUPERNOVAS_DIR);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let supernovas_dir = env::var(SUPERNOVAS_DIR).ok().map(PathBuf::from);

    let supernovas_include = if supernovas_dir.is_some() {
        supernovas_dir.as_ref().unwrap().join("include")
    } else {
        PathBuf::from("vendor/SuperNOVAS/include")
    };

    gen_bindings(&supernovas_include);

    #[cfg(feature = "novas-src")]
    let supernovas_dir = supernovas_dir.or_else(|| {
        let downloaded = out_path.join("supernovas");
        if !downloaded.exists() {
            download_supernovas(&out_path);
        }
        Some(out_path)
    });

    let supernovas_dir = match supernovas_dir {
        Some(dir) => {
            if !dir.exists() {
                println!("cargo:warning={}", format!("`supernovas_dir` does not point to a valid directory: {}", dir.display()));
                return;
            }
            dir
        },
        None => {
            println!("cargo:warning={}", format!("`supernovas_dir` does not point to a valid directory. Please set the {} environment variable or use `novas-src` feature.", SUPERNOVAS_DIR));
            return;
        }
    };

    println!("cargo:rustc-link-lib=static=cspice");
    println!("cargo:rustc-link-lib=static=calceph");

    #[cfg(feature = "novas-src")]
    build_supernovas(&supernovas_dir);

    let supernovas_lib = supernovas_dir.join("lib");
    let supernovas_include = supernovas_dir.join("include");

    println!("cargo:rustc-link-search=native={}", supernovas_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=supernovas");
    println!("cargo:include={}", supernovas_include.to_str().unwrap());
}

#[cfg(feature = "novas-src")]
fn download_supernovas(dst: &PathBuf) {
    let supernovas_version = "1.4.0";
    let url = format!("https://github.com/Smithsonian/SuperNOVAS/archive/refs/tags/v{}.tar.gz", supernovas_version);

    let body = reqwest::blocking::get(url)
        .expect("Failed to download supernovas archive")
        .bytes()
        .unwrap();

    let download_target = dst.join("supernovas.tar.gz");
    std::fs::write(download_target, body).unwrap();
    
    // Extract package based on platform
    let output = Command::new("tar")
        .arg("-xzf")
        .arg("supernovas.tar.gz")
        .current_dir(dst)
        .output()
        .expect("Failed to extract archive with tar");
    
    if !output.status.success() {
        panic!("Failed to extract archive: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Move the extracted directory to the destination
    let from = dst.join(format!("SuperNOVAS-{}", supernovas_version));
    let to = dst.join("supernovas");
    if to.exists() {
        fs::remove_dir_all(&to).expect("Failed to remove existing supernovas directory");
    }
    fs::rename(&from, &to).expect("Failed to rename extracted directory");

    // 将 vendor/SuperNOVAS/src 覆盖到 to.join("src")
    let src_dir = to.join("src");
    if src_dir.exists() {
        fs::remove_dir_all(&src_dir).expect("Failed to remove existing src directory");
    }
    fs::create_dir_all(&src_dir).expect("Failed to create src directory");
    let vendor_src_dir = PathBuf::from("vendor/SuperNOVAS/src");
    for entry in fs::read_dir(vendor_src_dir).expect("Failed to read vendor src directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() {
            fs::copy(&path, src_dir.join(path.file_name().unwrap())).expect("Failed to copy file to src directory");
        }
    }
}

#[cfg(feature = "novas-src")]
fn build_supernovas(supernovas_dir: &PathBuf) {
    let supernovas_dir = supernovas_dir.join("supernovas");
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib = dst.join("lib");
    let target = env::var("TARGET").unwrap();

    // Detect if we're building in debug or release mode
    let is_debug = env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true";

    let mut cfg = Build::new();

    if let Some(include) = std::env::var_os("DEP_CSPICE_INCLUDE") {
        cfg.include(include);
    }    if let Some(include) = std::env::var_os("DEP_CALCEPH_INCLUDE") {
        cfg.include(include);
    }

    cfg.warnings(false).out_dir(&lib).include(supernovas_dir.join("include"));

    let src_files: Vec<_> = fs::read_dir(supernovas_dir.join("src"))
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

    if target.contains("windows") {
        // Use appropriate runtime library based on build profile
        let (runtime_lib, runtime_flag) = if is_debug {
            ("msvcrtd", "/MDd")  // Debug runtime
        } else {
            ("msvcrt", "/MD")    // Release runtime
        };
        
        cfg.flag_if_supported("/std:c11")    // Use C11 standard
            .flag_if_supported("/MP")         // Multi-processor compilation
            .flag_if_supported("/O2")         // Optimization level 2
            .flag_if_supported(runtime_flag)  // Set runtime library
            .define("restrict", "")           // Remove restrict keyword
            .define("strcasecmp", "_stricmp") // MSVC uses _stricmp instead of strcasecmp
            .define("strncasecmp", "_strnicmp") // MSVC uses _strnicmp instead of strncasecmp
            .define("_CRT_SECURE_NO_WARNINGS", "") // Disable MSVC security warnings
            .define("_CRT_NONSTDC_NO_DEPRECATE", ""); // Disable non-standard function warnings
        
        // Link appropriate MSVC runtime libraries
        println!("cargo:rustc-link-lib={}", runtime_lib);
        println!("cargo:rustc-link-lib=legacy_stdio_definitions");
        
        // Use /NODEFAULTLIB to prevent automatic linking of conflicting runtime
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:msvcrt.lib");
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:msvcrtd.lib");
        println!("cargo:rustc-link-arg=/DEFAULTLIB:{}.lib", runtime_lib);
    }

    cfg.compile("supernovas");
    let src_include = supernovas_dir.join("include");
    let dst_include = dst.join("include");
    fs::create_dir_all(&dst_include).unwrap();
    let headers = ["novas-calceph.h", "novas-cspice.h", "novas.h", "nutation.h", "solarsystem.h"];
    headers.iter().for_each(|doth| {
        fs::copy(src_include.join(doth), dst_include.join(doth)).unwrap();
    });
}

fn gen_bindings(include_dst: &PathBuf) {
    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Generate the bindings
    let mut builder = bindgen::Builder::default()
        .header(include_dst.join("novas-calceph.h").to_str().unwrap())
        .header(include_dst.join("novas-cspice.h").to_str().unwrap())
        .header(include_dst.join("novas.h").to_str().unwrap())
        .header(include_dst.join("nutation.h").to_str().unwrap())
        .header(include_dst.join("solarsystem.h").to_str().unwrap());

    builder = builder.clang_arg(format!("-I{}", include_dst.to_string_lossy()));

    if let Some(calceph_include) = env::var_os("DEP_CALCEPH_INCLUDE") {
        builder = builder.clang_arg(format!("-I{}", calceph_include.to_string_lossy()));
    } else {
        builder = builder.clang_arg("-Ivendor/calceph/include");
    }

    if let Some(cspice_include) = env::var_os("DEP_CSPICE_INCLUDE") {
        builder = builder.clang_arg(format!("-I{}", cspice_include.to_string_lossy()));
    }

    builder = builder.blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .derive_default(true)
        .derive_debug(true);
    
    let bindings_path = dst.join("bindings.rs");
    
    let bindings = builder
        .generate()
        .expect("Unable to generate bindings for SuperNOVAS");

    bindings.write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
}
