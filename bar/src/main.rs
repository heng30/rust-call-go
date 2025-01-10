use std::ffi::c_int;

extern "C" {
    fn GoAdd(a: c_int, b: c_int) -> c_int;
    fn SayHi();
}

fn main() {
    unsafe {
        SayHi();

        let sum = GoAdd(5, 3);
        println!("Sum from Go: {}", sum);
    }
}
