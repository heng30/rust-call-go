#[allow(unused_imports)]
#[allow(dead_code)]

use std::ffi::c_int;

extern "C" {
    fn GoAdd(a: c_int, b: c_int) -> c_int;
    fn SayHi();
}

fn main() {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    unsafe {
        SayHi();

        let sum = GoAdd(5, 3);
        println!("Sum from Go: {}", sum);
    }

    #[cfg(target_os = "windows")]
    {
        use std::env;
        use std::path::Path;
        use libloading::{Library, Symbol};

        let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_path = Path::new(&out_dir).join("..").join("build").join(
            "libgo-shared.so",
        );

        let lib_path = format!(
            "{}",
            lib_path.into_os_string().as_os_str().to_str().unwrap()
        );

        println!("shared lib path: {}", lib_path);

        unsafe {
            let lib = Library::new(&lib_path)?;
            let sayhi_func: Symbol<unsafe extern "C" fn(u32, u32) -> u32> = lib.get(b"SayHi")
                .unwrap();

            sayhi_func(3, 5);

            let add_func: Symbol<unsafe extern "C" fn(u32, u32) -> u32> = lib.get(b"GoAdd")
                .unwrap();

            let result = add_func(3, 5);
            println!("Sum from Go: {}", result);
        }
    }
}
