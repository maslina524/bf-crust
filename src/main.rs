#![no_std]
#![no_main]

use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;

pub mod interpreter;
use interpreter::run;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe extern "C" {
    unsafe fn printf(fmt: *const c_char, ...) -> c_int;
    unsafe fn exit(status: i32) -> !;
}

#[no_mangle]
pub unsafe fn _start() -> i32 {
    printf("Brainfuck Interpreter in Crust!!!\n\0".as_ptr() as *const i8);

    let source = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.>>>++++++++[<++++>-]<.>>>++++++++++[<+++++++++>-]<---.<<<<.+++.------.--------.>>+.>++++++++++.";
    run(source.as_ptr() as *const u8, source.len());

    exit(0);
}