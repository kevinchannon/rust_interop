use libc::{size_t, c_char};
use std::{cmp, ptr};


pub mod detail;

///////////////////////////////////////////////////////////////////////////////

type ResultCode = u64;

pub const RC_OK: ResultCode = 0;
pub const RC_ERROR: ResultCode = 1;

type UserId = u64;
type UserHandle = u64;

///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn add_user(id: u64) -> UserHandle {
  detail::create_user(id)
}

///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn set_user_name(h: &UserHandle, name_as_null_term_chars: *const c_char) -> ResultCode {
  detail::set_user_name(h, detail::str_from_null_term_chars(name_as_null_term_chars))
}

///////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub unsafe extern "C" fn get_user_name(h: &UserHandle, buf: *mut c_char, max_len: size_t) -> ResultCode {
  detail::null_term_chars_from_str(detail::get_user_name(h), buf, max_len)
}

///////////////////////////////////////////////////////////////////////////////




