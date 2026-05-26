use core::ffi::{c_char, c_int};

unsafe extern "C" {
    unsafe fn printf(fmt: *const c_char, ...) -> c_int;
}

pub unsafe fn run(source: &str) -> () {
    for &byte in source.as_bytes() {
        printf("%c\0".as_ptr() as *const i8, byte as c_int);
    }
}