#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::ffi::c_int;

#[cfg(any(target_os = "linux", target_os = "macos"))]
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
        use libloading::{Library, Symbol};

        let out_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let lib_path = std::path::Path::new(&out_dir).join("..").join(
            "libgo-shared.dll",
        );

        let lib_path = format!(
            "{}",
            lib_path.into_os_string().as_os_str().to_str().unwrap()
        );

        println!("shared lib path: {}", lib_path);

        unsafe {
            let lib = Library::new(&lib_path).unwrap();
            let sayhi_func: Symbol<unsafe extern "C" fn()> = lib.get(b"SayHi").unwrap();

            sayhi_func();

            let add_func: Symbol<unsafe extern "C" fn(u32, u32) -> u32> = lib.get(b"GoAdd")
                .unwrap();

            let result = add_func(3, 5);
            println!("Sum from Go: {}", result);
        }
    }
}
