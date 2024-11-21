use std::path::PathBuf;

use llvmlibc_build::cmake;
use llvmlibc_build::config::Config;

fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");
    let dir_path = PathBuf::from(dir);
    let libc_path = dir_path.join("src/libc");
    let compiler_rt_path = dir_path.join("src/compiler-rt");
    let cfg = Config::new_with_scudo(libc_path, compiler_rt_path);
    if std::env::var("CXX").is_err() {
        std::env::set_var("CXX", "clang++");
    }
    if std::env::var("CC").is_err() {
        std::env::set_var("CC", "clang");
    }
    let mut cmake_cfg = cmake::Config::from(&cfg);
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    cmake_cfg.target(&format!("{}-unknown-linux-gnu", arch));
    let root = cmake_cfg.build_target("libc").build();
    cmake_cfg.build_target("libm").build();
    cmake_cfg
        .build_target("libc.startup.linux.crt1.__relocatable__")
        .build();
    cmake_cfg.build_target("libc.startup.linux.crti").build();
    cmake_cfg.build_target("libc.startup.linux.crtn").build();

    let startup_dir = root.join("build").join("startup").join("linux");
    let crt1 = startup_dir.join("crt1.o");
    let crti = startup_dir.join("CMakeFiles").join("libc.startup.linux.crti.dir").join("crti.cpp.o");
    let crtn = startup_dir.join("CMakeFiles").join("libc.startup.linux.crtn.dir").join("crtn.cpp.o");

    let lib_path = root.join("build").join("lib");
    let startup_path = lib_path.join("libstartup.a");
    std::process::Command::new("ar")
        .arg("rs")
        .arg(&startup_path)
        .args(&[
            crt1.to_str().unwrap(),
            crti.to_str().unwrap(),
            crtn.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=c");
    println!("cargo:rustc-link-lib=static=m");
    println!("cargo:rustc-link-lib=static=startup");
}
