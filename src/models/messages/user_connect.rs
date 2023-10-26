use actix::prelude::*;

use crate::models::auth::User;

use super::{
  UserChat,
  traits::{ToUserChat, ToServerChat}
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserConnect<T: Actor> {
  pub user: User,
  pub socket_addr: Addr<T>,
}
impl<T: Actor> ToUserChat for UserConnect<T> {
  fn to_user_message(&self) -> UserChat {
    UserChat {
      username: self.user.name.to_owned(),
      message: format!("{} connected!", self.user.name.to_string()),
    }
  }
}
impl<T: Actor> ToServerChat for UserConnect<T> {}
