use crate::models::{messages::{ServerChat, UserChat}, Room};

pub trait ToServerChat {
  fn to_user_message(&self) -> UserChat;

  fn to_chat_message(&self, room: &Room, author: &str) -> ServerChat {
    let user_message = self.to_user_message();
    let room_id = room.id;
    let mut users = Vec::from_iter(room.users.values().map(|v| v.to_owned()));
    users.sort_unstable_by_key(|v| v.join_date);
    let message = format!("{}{}", author, user_message.message);

    ServerChat { room_id, users, message }
  }
}
