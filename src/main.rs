extern crate libc;

mod go {
    use libc::c_int;

    #[link(name = "buildmodes")]
    extern {
        fn Multiply(a: c_int, b: c_int) -> c_int;
    }

    pub fn multiply(a: c_int, b: c_int) -> c_int {
        unsafe { Multiply(a, b) }
    }
}

fn main() {
    println!("{}", go::multiply(4, 5));
}
