use core::ffi::{c_char, c_int, c_uint};

unsafe extern "C" {
    unsafe fn printf(fmt: *const c_char, ...) -> c_int;
    unsafe fn getchar() -> c_int;
    unsafe fn exit(status: i32) -> !;
}

pub unsafe fn run(ptr: *const u8, len: usize) {
    let mut mem = [0u8; 256];
    let mut mem_i: usize = 0;
    let mut i = 0;

    while i < len {
        let byte = *ptr.add(i);
        match byte {
            43 => { // '+'
                let val = *mem.get_unchecked(mem_i);
                let new_val = if val == 255 { 0 } else { val + 1 };
                *mem.get_unchecked_mut(mem_i) = new_val;
                i += 1;
            }
            44 => { // ','
                let ch = getchar() as u8;
                *mem.get_unchecked_mut(mem_i) = ch;
                i += 1;
            }
            45 => { // '-'
                let val = *mem.get_unchecked(mem_i);
                let new_val = if val == 0 { 255 } else { val - 1 };
                *mem.get_unchecked_mut(mem_i) = new_val;
                i += 1;
            }
            46 => { // '.'
                let val = *mem.get_unchecked(mem_i);
                printf("%c\0".as_ptr() as *const i8, val as c_uint);
                i += 1;
            }
            60 => { // '<'
                if mem_i == 0 {
                    printf("mem_i out of range\n\0".as_ptr() as *const i8);
                    exit(1);
                }
                mem_i -= 1;
                i += 1;
            }
            62 => { // '>'
                if mem_i == 255 {
                    printf("mem_i out of range\n\0".as_ptr() as *const i8);
                    exit(1);
                }
                mem_i += 1;
                i += 1;
            }
            91 => { // '['
                if *mem.get_unchecked(mem_i) == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        i += 1;
                        if i >= len {
                            printf("unmatched '['\n\0".as_ptr() as *const i8);
                            exit(1);
                        }
                        match *ptr.add(i) {
                            91 => depth += 1,
                            93 => depth -= 1,
                            _ => {}
                        }
                    }
                }
                i += 1;
            }
            93 => { // ']'
                if *mem.get_unchecked(mem_i) != 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        if i == 0 {
                            printf("unmatched ']'\n\0".as_ptr() as *const i8);
                            exit(1);
                        }
                        i -= 1;
                        match *ptr.add(i) {
                            91 => depth -= 1,
                            93 => depth += 1,
                            _ => {}
                        }
                    }
                } else {
                    i += 1;
                }
            }
            _ => {
                printf("unknown symb: %c = %d\n\0".as_ptr() as *const i8, byte as c_int, byte as c_int);
                exit(1);
            }
        }
    }
}