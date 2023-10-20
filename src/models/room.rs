use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use super::{WsMessage, UserDisconnect, UserConnect};

pub struct Room {
  pub id: Uuid,
  users: HashMap<Uuid, Recipient<WsMessage>>,
}

impl Default for Room {
  fn default() -> Self {
    Room {
      id: Uuid::new_v4(),
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
    self.users.insert(msg.user_id, msg.user_addr);
    self.send_message(format!("(Room {}): {} connected!", &self.id, &msg.user_id).as_str(), None);
  }
}

impl Handler<UserDisconnect> for Room {
  type Result = ();

  fn handle(&mut self, msg: UserDisconnect, _: &mut Self::Context) -> Self::Result {
    if let Some(_user) = self.users.remove(&msg.user_id) {
      self.send_message(format!("(Room {}): {} disconnected", &self.id, &msg.user_id).as_str(), Some(&msg.user_id));
    }
  }
}

impl Handler<WsMessage> for Room {
  type Result = ();

  fn handle(&mut self, msg: WsMessage, _: &mut Self::Context) -> Self::Result {
    self.send_message(&msg.0, None)
  }
}

impl Room {
  fn send_message(&self, message: &str, id_to_skip: Option<&Uuid>) {
    self.users.iter().for_each(|user| {
      if Some(user.0) != id_to_skip {
        let _ = user.1.do_send(WsMessage(message.to_string()));
      }
    });
  }
}
