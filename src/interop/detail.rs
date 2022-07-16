use std::ops::{Deref, DerefMut};
use libc::c_char;
use std::ffi::CStr;
use std::sync::{Mutex, Once};
use std::borrow::BorrowMut;

use super::*;

///////////////////////////////////////////////////////////////////////////////

pub fn str_from_null_term_chars(null_term_chars: *const c_char) -> &'static str {
  let c_str = unsafe {
    assert!(!null_term_chars.is_null());
    CStr::from_ptr(null_term_chars)
  };
  
  c_str.to_str().unwrap()
}

pub fn null_term_chars_from_str(s: &str, buf: *mut c_char, max_len: size_t) -> ResultCode {
  let out_bytes = s.as_bytes();
  let out_size = cmp::min(max_len as usize, out_bytes.len() + 1);
  let terminator_idx = (out_size - 1) as isize;
        
  unsafe {
        for i in 0..terminator_idx as isize {
            ptr::write(buf.offset(i), out_bytes[i as usize] as i8);
        }
        
        // Write the null terminator
        ptr::write(buf.offset(terminator_idx), '\0' as i8);
    }
    
  return RC_OK;
}

///////////////////////////////////////////////////////////////////////////////

pub struct User {
  id: u64,
  name: String
}

impl User {
  pub fn new(id: UserId) -> Self {
    Self { id: id , name: String::new()}
  }
}

pub struct UserList{
  users: Vec<User>
}

impl UserList {
  pub fn create_user(&mut self, id: UserId) -> UserHandle {
    self.users.push(User::new(id));
    return id as UserHandle;
  }
}

impl Deref for UserList {
  type Target = Vec<User>;

  fn deref(&self) -> &Vec<User> { &self.users}
}

impl DerefMut for UserList {
  fn deref_mut(&mut self) -> &mut Self::Target { &mut self.users}
}


static mut STD_ONCE_COUNTER: Option<Mutex<UserList>> = None;
static INIT: Once = Once::new();

fn user_list<'a>() -> &'a Mutex<UserList> {
    INIT.call_once(|| {
        // Since this access is inside a call_once, it is safe
        unsafe {
            *STD_ONCE_COUNTER.borrow_mut() = Some(Mutex::new(UserList{users: Vec::<User>::new()}));
        }
    });
    // As long as this function is the only place with access to the static variable,
    // giving out read-only borrow here is safe because it is guaranteed no more mutable
    // references will exist at this point or in the future.
    unsafe { STD_ONCE_COUNTER.as_ref().unwrap() }
}

// static mut USERS: UserList = UserList{users: Vec::<User>::new()};

pub fn create_user(id: UserId) -> UserHandle {
  (*user_list().lock().unwrap()).create_user(id)
}

pub fn set_user_name(h: UserHandle, name: &str) -> ResultCode {
  match (*user_list().lock().unwrap()).iter().position(|u| u.id == h){
    Some(pos) => {
      (*user_list().lock().unwrap())[pos].name = name.to_string();
      return RC_OK;
      },
    None => {return RC_ERROR;}
  }
}

pub fn get_user_name(h: UserHandle) -> &'static str {
  match (*user_list().lock().unwrap()).iter().position(|u| u.id == h){
    Some(pos) => &(*user_list().lock().unwrap()[pos].name)[..],
    None => ""
  }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn add_a_user_returns_id_as_handle_handle() {  
    unsafe {
     assert_eq!(123, create_user(123));
     }
  }
  
  #[test]
  fn set_and_get_user_name_with_valid_handle_returns_name(){
    unsafe {
      assert_eq!(RC_OK, set_user_name(123, "human 1"));
      assert_eq!("human 1", get_user_name(123));
    }
  } 
}

///////////////////////////////////////////////////////////////////////////////

