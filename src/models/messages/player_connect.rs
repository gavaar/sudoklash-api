use actix::prelude::*;
use uuid::Uuid;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PlayerConnect {
  pub selection: u16,
}

#[derive(Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct Player {
  pub user_id: String,
  pub selection: u16,
}
impl Player {
  pub fn empty() -> Player {
    Player {
      user_id: Uuid::nil().to_string(),
      selection: 0u16,
    }
  }
}
