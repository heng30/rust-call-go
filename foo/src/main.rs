use std::ffi::c_int;

extern "C" {
    fn GoAdd(a: c_int, b: c_int) -> c_int;
    fn SayHi();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        let lib_path = Path::new(&out_dir)
            .join("..")
            .join("build")
            .join("libgo-shared.so");

        let lib_path = format!("{}", lib_path.to_str().unwrap());

        unsafe {
            let lib = Library::new(&lib_path)?;
            let sayhi_func: Symbol<unsafe extern fn(u32, u32) -> u32> = lib.get(b"SayHi")?;
            sayhi_func(3, 5) ;

            let add_func: Symbol<unsafe extern fn(u32, u32) -> u32> = lib.get(b"GoAdd")?;
            let result =  add_func(3, 5) ;
            println!("Sum from Go: {}", result);
        }
    }

    Ok(())
}
