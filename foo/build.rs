use std::env;
use std::path::Path;

fn main() {

    #[cfg(target_os = "linux")]
    {
        let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_dir = Path::new(&out_dir).join("..").join("build");
        println!("cargo:rustc-link-arg=-L");
        println!("cargo:rustc-link-arg={}", lib_dir.to_str().unwrap());
        println!("cargo:rustc-link-lib=go-shared");
    }

    #[cfg(target_os = "macos")]
    {
        panic!("unsupport call Go functions from shared lib on macos");
    }
}
