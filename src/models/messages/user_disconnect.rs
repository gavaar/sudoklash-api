use actix::prelude::*;

use super::{
  UserChat,
  traits::ToServerChat,
};

#[derive(Message)]
#[rtype(result = "()")]
pub struct UserDisconnect {
  pub user_id: String,
  pub username: String,
}
impl ToServerChat for UserDisconnect {
  fn to_user_message(&self) -> UserChat {
    UserChat {
      username: self.username.to_owned(),
      message: format!("{} just disconnected...", self.username.to_string()),
    }
  }
}
