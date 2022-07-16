use libc::{size_t, c_int, c_char};
use std::{cmp, ptr};


pub mod detail;


///////////////////////////////////////////////////////////////////////////////
//
// C-interface parts
//
///////////////////////////////////////////////////////////////////////////////

type ResultCode = u64;

pub const RC_OK: ResultCode = 0;
pub const RC_ERROR: ResultCode = 1;

///////////////////////////////////////////////////////////////////////////////

type UserId = u64;
type UserHandle = u64;

///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn add_user(id: u64) -> UserHandle {
  detail::create_user(id)
}

///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn set_user_name(h: UserHandle, name_as_null_term_chars: *const c_char) -> ResultCode {
  detail::set_user_name(h, detail::str_from_null_term_chars(name_as_null_term_chars))
}

///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn get_user_name(h: UserHandle, buf: *mut c_char, max_len: size_t) -> ResultCode {
  detail::null_term_chars_from_str(detail::get_user_name(h), buf, max_len)
}

///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn get_int() -> i32 { 2 }

///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn fill_string(buf: *mut u8, max_len: size_t) -> c_int {
  let out_bytes = detail::get_string().as_bytes();
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

///////////////////////////////////////////////////////////////////////////////




