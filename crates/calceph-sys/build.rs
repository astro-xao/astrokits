use cmake::Config;
use std::path::PathBuf;
use std::{env, fs};
use std::process::Command;

const CALCEPH_DIR: &str = "CALCEPH_DIR";

fn main() {
    println!("cargo:rerun-if-env-changed={}", CALCEPH_DIR);

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let calceph_dir = env::var(CALCEPH_DIR).ok().map(PathBuf::from);

    let calceph_include = if calceph_dir.is_some() {
        calceph_dir.as_ref().unwrap().join("include")
    } else {
        PathBuf::from("vendor/calceph/include")
    };

    gen_bindings(&calceph_include);

    #[cfg(feature = "calceph-src")]
    let calceph_dir = calceph_dir.or_else(|| {
        let downloaded = out_path.join("cacleph");
        if !downloaded.exists() {
            download_calceph(&out_path);
        }
        Some(out_path)
    });

    let calceph_dir = match calceph_dir {
        Some(dir) => {
            if !dir.exists() {
                println!("cargo:warning={}", format!("`calceph_dir` does not point to a valid directory: {}", dir.display()));
                return;
            }
            dir
        },
        None => {
            println!("cargo:warning={}", format!("`calceph_dir` does not point to a valid directory. Please set the {} environment variable or use `calceph-src` feature.", CALCEPH_DIR));
            return;
        }
    };

    #[cfg(feature = "calceph-src")]
    build_calceph(&calceph_dir);

    let calceph_lib = calceph_dir.join("lib");
    let calceph_include = calceph_dir.join("include");

    println!("cargo:rustc-link-search=native={}", calceph_lib.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=calceph");
    println!("cargo:include={}", calceph_include.to_str().unwrap());
}

#[cfg(feature = "calceph-src")]
fn download_calceph(dst: &PathBuf) {
    let calceph_version = "4_0_5";
    let url = format!("https://gitlab.obspm.fr/imcce_calceph/calceph/-/archive/calceph_{}/calceph-calceph_{}.tar.gz", calceph_version, calceph_version);

    let body = reqwest::blocking::get(url)
        .expect("Failed to download calceph archive")
        .bytes()
        .unwrap();

    let download_target = dst.join("calceph.tar.gz");
    std::fs::write(download_target, body).unwrap();
    
    // Extract package based on platform
    let output = Command::new("tar")
        .arg("-xzf")
        .arg("calceph.tar.gz")
        .current_dir(dst)
        .output()
        .expect("Failed to extract archive with tar");
    
    if !output.status.success() {
        panic!("Failed to extract archive: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Move the extracted directory to the destination
    let from = dst.join(format!("calceph-calceph_{}", calceph_version));
    let to = dst.join("calceph");
    if to.exists() {
        fs::remove_dir_all(&to).expect("Failed to remove existing calceph directory");
    }
    fs::rename(&from, &to).expect("Failed to rename extracted directory");
}

#[cfg(feature = "calceph-src")]
fn build_calceph(cacleph_dir: &PathBuf) {
    let target = env::var("TARGET").unwrap();
    // Build the CMake project using NMake Makefiles generator
    let mut cfg = Config::new(cacleph_dir.join("calceph"));
    cfg.define("ENABLE_FORTRAN", "OFF");
    if target.contains("msvc")
    {
        cfg.generator("NMake Makefiles");
    }
    cfg.build();
}

fn gen_bindings(include_dst: &PathBuf) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Generate the bindings
    let bindings = bindgen::Builder::default()
        .header(include_dst.join("calceph.h").to_str().unwrap())
        .generate()
        .expect("Unable to generate bindings");

    let bindings_path = out_path.join("bindings.rs");

    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
}