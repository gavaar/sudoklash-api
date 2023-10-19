use std::collections::HashMap;

use actix::{Recipient, Context, Actor, Handler};
use uuid::Uuid;

use super::{WsMessage, UserDisconnect, UserConnect};

pub struct Room {
  users: HashMap<Uuid, Recipient<WsMessage>>,
}

impl Default for Room {
  fn default() -> Self {
    Room {
      users: HashMap::new(),
    }
  }
}

impl Actor for Room {
  type Context = Context<Self>;
}

impl Handler<UserConnect> for Room {
  type Result = ();

  fn handle(&mut self, msg: UserConnect, _: &mut Self::Context) -> Self::Result {
    self.send_message(format!("C: {} connected!", &msg.user_id).as_str(), None);
    self.users.insert(msg.user_id, msg.user_addr);
  }
}

impl Handler<UserDisconnect> for Room {
  type Result = ();

  fn handle(&mut self, msg: UserDisconnect, _: &mut Self::Context) -> Self::Result {
    if let Some(_user) = self.users.remove(&msg.user_id) {
      self.send_message(format!("X: {} disconnected", msg.user_id).as_str(), Some(&msg.user_id));
    }
  }
}

impl Room {
  fn send_message(&self, message: &str, id_to_skip: Option<&Uuid>) {
    self.users.iter().for_each(|user| {
      if Some(user.0) != id_to_skip {
        let _ = user.1.send(WsMessage(message.to_owned()));
      }
    });
  }
}
