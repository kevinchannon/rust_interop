use std::ops::{Deref, DerefMut};

use super::*;

pub fn get_string() -> &'static str {
  "I'm a little teapot"
}

impl User {
  pub fn new(id: UserId) -> Self {
    Self { id: id }
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

