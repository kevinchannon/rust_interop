use std::ops::{Deref, DerefMut};

use super::*;

pub fn get_string() -> &'static str {
  "I'm a little teapot"
}

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

static mut USERS: UserList = UserList{users: Vec::<User>::new()};

pub unsafe fn create_user(id: UserId) -> UserHandle {
  USERS.create_user(id)
}

pub unsafe fn set_user_name(h: UserHandle, name: &str) -> ResultCode {
  match USERS.iter().position(|u| u.id == h){
    Some(pos) => { USERS[pos].name = name.to_string(); return RC_OK; },
    None => {return RC_ERROR;}
  }
}

pub unsafe fn get_user_name(_h: UserHandle) -> &'static str {
  "wibble"
}

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
    fn set_user_name_with_valid_name_return_success(){
      unsafe {
        assert_eq!(RC_OK, set_user_name(123, "human 1"));
      }
    }
    
    #[test]
    fn get_user_name_with_valid_handle_returns_name(){
      unsafe {
        assert_eq!("human 1", get_user_name(123));
      }
    }
}

