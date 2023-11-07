use actix::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::models::room::RoomUser;

#[derive(Message, Serialize, Clone)]
#[rtype(result = "()")]
pub struct ServerChat {
  pub room_id: Uuid,
  pub users: Vec<RoomUser>,
  pub message: String,
}
