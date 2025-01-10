use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = Path::new(&out_dir)
        .join("..")
        .join("build");

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        println!("cargo:rustc-link-arg=-L");
        println!("cargo:rustc-link-arg={}", lib_dir.to_str().unwrap());
        println!("cargo:rustc-link-lib=go-static");
    }

    #[cfg(target_os = "windows")]
    {
        // let lib_dir = lib_dir.join("libgo-static.a");
        // println!("cargo:rustc-link-arg={}", lib_dir.to_str().unwrap());
        println!("cargo:rustc-link-arg= /d/a/rust-call-go/rust-call-go/build/libgo-static.lib");
    }
}
