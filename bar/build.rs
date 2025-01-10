fn main() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        use std::env;
        use std::path::Path;

        let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_dir = Path::new(&out_dir).join("..").join("build");
        println!("cargo:rustc-link-arg=-L");
        println!("cargo:rustc-link-arg={}", lib_dir.to_str().unwrap());
        println!("cargo:rustc-link-lib=go-static");
    }

    #[cfg(target_os = "windows")]
    {
        panic!("unsupport call Go functions from static lib on windows");
    }
}
