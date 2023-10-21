use std::collections::HashMap;

use actix::prelude::*;
use uuid::Uuid;

use super::{messages::{ServerChat, UserDisconnect, UserConnect, UserChat, traits::ToServerChat, Player, Tick}, Game, UserSocket, GameSocket, Turn, GameStatus};

pub struct Room {
  pub id: Uuid,
  pub game: Game,
  pub users: HashMap<Uuid, Addr<UserSocket>>,
  pub gamers: HashMap<Uuid, Addr<GameSocket>>,
}
impl Default for Room {
  fn default() -> Self {
    Room {
      id: Uuid::new_v4(),
      game: Game::new(),
      users: HashMap::new(),
      gamers: HashMap::new(),
    }
  }
}

impl Actor for Room {
  type Context = Context<Self>;
}

impl Handler<Player> for Room {
  type Result = ();

  fn handle(&mut self, player_message: Player, _: &mut Self::Context) -> Self::Result {
    println!("player: {:#?}", player_message.to_owned());
    self.game.assing_player(player_message);
    self.send_game_update();
  }
}

impl Handler<Turn> for Room {
  type Result = ();

  fn handle(&mut self, turn: Turn, _: &mut Self::Context) -> Self::Result {
    self.game.play_turn(turn);
    self.send_game_update();
    if self.game.game_status == GameStatus::Ended {
      self.close_game();
    }
  }
}

impl Handler<UserConnect<GameSocket>> for Room {
  type Result = ();

  fn handle(&mut self, user_connect_game_socket: UserConnect<GameSocket>, _: &mut Self::Context) -> Self::Result {
    self.gamers.insert(user_connect_game_socket.user_id, user_connect_game_socket.socket_addr);
    self.send_game_update();
  }
}

impl Handler<UserConnect<UserSocket>> for Room {
  type Result = ();

  fn handle(&mut self, user_connect_user_socket: UserConnect<UserSocket>, _: &mut Self::Context) -> Self::Result {
    let bot_message = user_connect_user_socket.to_chat_message(self, "");

    self.users.insert(user_connect_user_socket.user_id, user_connect_user_socket.socket_addr);

    self.send_message(bot_message, None);
  }
}

impl Handler<UserDisconnect> for Room {
  type Result = ();

  fn handle(&mut self, disconnect_msg: UserDisconnect, _: &mut Self::Context) -> Self::Result {
    if let Some(_user) = self.users.remove(&disconnect_msg.user_id) {
      let message = disconnect_msg.to_chat_message(self, disconnect_msg.user_id.to_string().as_str());
      self.send_message(message, Some(&disconnect_msg.user_id));
    }
  }
}

impl Handler<UserChat> for Room {
  type Result = ();

  fn handle(&mut self, connect_msg: UserChat, _: &mut Self::Context) -> Self::Result {
    self.send_message(connect_msg.to_chat_message(self, connect_msg.user_id.to_string().as_str()), None);
  }
}

impl Room {
  fn send_message(&self, message: ServerChat, id_to_skip: Option<&Uuid>) {
    self.users.iter().for_each(|user| {
      if Some(user.0) != id_to_skip {
        let _ = user.1.do_send(message.to_owned());
      }
    });
  }

  fn send_game_update(&self) {
    self.gamers.iter().for_each(|gamer| {
      gamer.1.do_send(Tick(self.game.to_owned()))
    });
  }

  fn close_game(&self) {
    self.gamers.iter().for_each(|gamer| {
      gamer.1.do_send(UserDisconnect { user_id: gamer.0.to_owned() });
    });
  }
}
