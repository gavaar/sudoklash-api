use actix::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Message, Serialize, Clone)]
#[rtype(result = "()")]
pub struct ServerChat {
  pub room_id: Uuid,
  pub users: Vec<String>,
  pub message: String,
}
