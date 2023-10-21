use actix::prelude::*;
use uuid::Uuid;

use super::{
  UserChat,
  traits::{ToUserChat, ToServerChat}
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserDisconnect {
  pub user_id: Uuid,
}
impl ToUserChat for UserDisconnect {
  fn to_user_message(&self) -> UserChat {
    UserChat {
      user_id: self.user_id,
      message: format!("{} just disconnected...", self.user_id.to_string()),
    }
  }
}
impl ToServerChat for UserDisconnect {}
