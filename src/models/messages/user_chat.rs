use actix::prelude::*;
use serde::Deserialize;

use super::traits::ToServerChat;

#[derive(Message, Deserialize, Clone)]
#[rtype(result = "()")]
pub struct UserChat {
  pub username: String,
  pub message: String,
}
impl ToServerChat for UserChat {
  fn to_user_message(&self) -> UserChat { self.to_owned() }
}
