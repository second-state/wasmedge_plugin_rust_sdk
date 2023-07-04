use std::path::PathBuf;

macro_rules! env_path {
    ($env_var:literal) => {
        std::env::var_os($env_var).map(PathBuf::from)
    };
}

fn main() {
    let  lib_dir = env_path!("WASMEDGE_LIB_DIR").expect(
        "[wasmedge-sys-ffi] Failed to locate the required header and/or library file. Please specify via the environment variable `WASMEDGE_LIB_DIR`.",
    );

    println!("cargo:rustc-env=LD_LIBRARY_PATH={}", lib_dir.display());
    println!("cargo:rustc-link-search={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=wasmedge");
}
