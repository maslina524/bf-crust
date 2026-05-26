use core::ffi::{c_char, c_int};

unsafe extern "C" {
    unsafe fn printf(fmt: *const c_char, ...) -> c_int;
}

pub unsafe fn run(ptr: *const u8, len: usize) -> () {
    for i in 0..len {
        let byte = *ptr.add(i);
        printf("%c\0".as_ptr() as *const i8, byte as c_int);
    }
}