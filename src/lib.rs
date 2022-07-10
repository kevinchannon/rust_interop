use libc::{size_t, c_int};
use std::{cmp, ptr};

#[no_mangle]
pub extern "C" fn get_int() -> i32 { 2 }

#[no_mangle]
pub extern "C" fn get_string(buf: *mut u8, max_len: size_t) -> c_int {
  let out_bytes = "I'm a little teapot".as_bytes();
  let out_size = cmp::min(max_len as usize, out_bytes.len());
  unsafe {
        for i in 0..out_size as isize {
            ptr::write(buf.offset(i), out_bytes[i as usize]);
        }
    }
    
  out_size as c_int
}

