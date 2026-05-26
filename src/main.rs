#![no_std]
#![no_main]

use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    unsafe fn printf(fmt: *const c_char, ...) -> c_int;
    unsafe fn exit(status: i32) -> !;
}

#[no_mangle]
unsafe fn _start() -> i32 {
    printf("Hello, World!!!".as_ptr() as *const i8);
    exit(0);
}