use std::ffi::c_int;

extern "C" {
    fn GoAdd(a: c_int, b: c_int) -> c_int;
}

fn main() {
    unsafe {
        let sum = GoAdd(5, 3);
        println!("Sum from Go: {}", sum);
    }
}
