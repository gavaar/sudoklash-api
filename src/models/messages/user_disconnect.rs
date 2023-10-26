use actix::prelude::*;

use super::{
  UserChat,
  traits::{ToUserChat, ToServerChat}
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserDisconnect {
  pub user_id: String,
  pub username: String,
}
impl ToUserChat for UserDisconnect {
  fn to_user_message(&self) -> UserChat {
    UserChat {
      username: self.username.to_owned(),
      message: format!("{} just disconnected...", self.username.to_string()),
    }
  }
}
impl ToServerChat for UserDisconnect {}
