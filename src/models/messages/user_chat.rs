use actix::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

use super::traits::{ToUserChat, ToServerChat};

#[derive(Message, Deserialize, Clone)]
#[rtype(result = "()")]
pub struct UserChat {
  pub user_id: Uuid,
  pub message: String,
}
impl ToUserChat for UserChat {
  fn to_user_message(&self) -> UserChat { self.to_owned() }
}
impl ToServerChat for UserChat {}
