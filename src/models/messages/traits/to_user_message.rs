use crate::models::messages::UserChat;

pub trait ToUserChat {
  fn to_user_message(&self) -> UserChat;
}
