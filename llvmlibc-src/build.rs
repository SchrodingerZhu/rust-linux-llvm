use std::path::PathBuf;

use llvmlibc_build::cmake;
use llvmlibc_build::config::Config;

fn main() {
    println!("cargo:rerun-if-changed=src/cmake");
    println!("cargo:rerun-if-changed=src/compiler-rt");
    println!("cargo:rerun-if-changed=src/libc");

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
    let libc = cmake_cfg.build_target("libc").build();
    let libm = cmake_cfg.build_target("libm").build();
    let crt1 = cmake_cfg
        .build_target("libc.startup.linux.crt1.__relocatable__")
        .build();
    let crti = cmake_cfg.build_target("libc.startup.linux.crti").build();
    let crtn = cmake_cfg.build_target("libc.startup.linux.crtn").build();
}
