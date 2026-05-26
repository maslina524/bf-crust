use core::ffi::{c_char, c_int, c_uint};

unsafe extern "C" {
    unsafe fn printf(fmt: *const c_char, ...) -> c_int;
}

pub unsafe fn run(ptr: *const u8, len: usize) -> () {
    let mut mem = [0u8; 256];
    let mut mem_i: usize = 0;

    for i in 0..len {
        let byte = *ptr.add(i);
        match byte {
            43 => { // plus
                mem[mem_i] += 1;
            },
            45 => { // minus
                mem[mem_i] -= 1;
            },
            _ => {
                printf("unknown symb\n\0".as_ptr() as *const i8);
            }
        }
        printf("%c = %d\n\0".as_ptr() as *const i8, byte as c_int, byte as c_int);
    }
}