use crate::models::{messages::ServerChat, Room};

use super::ToUserChat;

pub trait ToServerChat: ToUserChat {
  fn to_chat_message(&self, room: &Room, author: &str) -> ServerChat {
    let user_message = self.to_user_message();
    let room_id = room.id;
    let users: Vec<String> = room.usernames.values().map(|v| v.to_owned()).collect();
    let message = format!("{}{}", author, user_message.message);

    ServerChat { room_id, users, message }
  }
}
