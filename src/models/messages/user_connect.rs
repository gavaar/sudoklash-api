use actix::prelude::*;
use uuid::Uuid;

use super::{
  UserChat,
  traits::{ToUserChat, ToServerChat}
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserConnect<T: Actor> {
  pub user_id: Uuid,
  pub socket_addr: Addr<T>,
}
impl<T: Actor> ToUserChat for UserConnect<T> {
  fn to_user_message(&self) -> UserChat {
    UserChat {
      user_id: self.user_id,
      message: format!("{} connected!", self.user_id.to_string()),
    }
  }
}
impl<T: Actor> ToServerChat for UserConnect<T> {}
