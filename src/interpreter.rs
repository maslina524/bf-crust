use core::ffi::{c_char, c_int, c_uint};

unsafe extern "C" {
    unsafe fn printf(fmt: *const c_char, ...) -> c_int;
    unsafe fn getchar() -> c_int;
    unsafe fn exit(status: i32) -> !;
}

pub unsafe fn run(ptr: *const u8, len: usize) -> () {
    let mut mem = [0u8; 256];
    let mut mem_i: usize = 0;

    for i in 0..len {
        let byte = *ptr.add(i);
        match byte {
            43 => { // plus
                let val = unsafe { *mem.get_unchecked(mem_i) };
                let new_val = if val == 255 { 0 } else { val + 1 };
                unsafe { *mem.get_unchecked_mut(mem_i) = new_val };
            },
            44 => { // comma
                let ch = getchar() as u8;
                unsafe { *mem.get_unchecked_mut(mem_i) = ch };
            },
            45 => { // minus
                let val = unsafe { *mem.get_unchecked(mem_i) };
                let new_val = if val == 0 { 255 } else { val - 1 };
                unsafe { *mem.get_unchecked_mut(mem_i) = new_val };
            },
            46 => { // dot
                let val = unsafe { *mem.get_unchecked(mem_i) };
                printf("%c\0".as_ptr() as *const i8, val as c_uint);
            },
            60 => { // less
                if mem_i == 0 {
                    printf("mem_i out of range\n\0".as_ptr() as *const i8);
                    exit(1);
                }
                mem_i -= 1;
            },
            62 => { // greater
                if mem_i == 255 {
                    printf("mem_i out of range\n\0".as_ptr() as *const i8);
                    exit(1);
                }
                mem_i += 1;
            },
            _ => {
                printf("unknown symb: %c = %d\n\0".as_ptr() as *const i8, byte as c_int, byte as c_int);
                exit(1);
            }
        }
    }
}