use libc::{size_t, c_int};
use std::{cmp, ptr};

#[no_mangle]
pub extern "C" fn get_int() -> i32 { 2 }

#[no_mangle]
pub extern "C" fn fill_string(buf: *mut u8, max_len: size_t) -> c_int {
  let out_bytes = get_string().as_bytes();
  let out_size = cmp::min(max_len as usize, out_bytes.len());
  let terminator_idx = (out_size - 1) as isize;
        
  unsafe {
        
        for i in 0..terminator_idx as isize {
            ptr::write(buf.offset(i), out_bytes[i as usize]);
        }
        
        // Write the null terminator
        ptr::write(buf.offset(terminator_idx), '\0' as u8);
    }
    
  out_size as c_int
}

fn get_string() -> &'static str {
  "I'm a little teapot"
}

