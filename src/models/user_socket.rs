use std::time::{Instant, Duration};

use actix::prelude::*;
use actix_web_actors::ws;
use uuid::Uuid;

use crate::models::{Room, UserConnect, UserDisconnect};

use super::WsMessage;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct UserSocket {
  user_id: Uuid,
  hb: Instant,
  room_addr: Addr<Room>,
}

impl UserSocket {
  pub fn new(user_id: Uuid, room_addr: Addr<Room>) -> UserSocket {
    UserSocket { user_id, hb: Instant::now(), room_addr }
  }

  pub fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
    ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
      if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
          println!("Coyo e su badre, se desconecto el menor!");

          ctx.stop();

          return;
      }

      ctx.ping(b"ping");
    });
  }
}

impl Actor for UserSocket {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    self.hb(ctx);

    let user_id = self.user_id;
    let user_addr = ctx.address().recipient();

    self.room_addr
      .send(UserConnect { user_id, user_addr })
      .into_actor(self)
      .then(|res, _, ctx| {
        match res {
          Ok(_) => (),
          _ => ctx.stop(),
        }
        fut::ready(())
      })
      .wait(ctx);
  }

  fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
    self.room_addr.do_send(UserDisconnect { user_id: self.user_id });
    Running::Stop
  }
}

impl Handler<WsMessage> for UserSocket {
  type Result = ();
  
  fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
    ctx.text(msg.0);
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UserSocket {
  fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    let msg = match item {
      Err(_) => {
        ctx.stop();
        return;
      }
      Ok(msg) => msg
    };

    match msg {
      ws::Message::Ping(ping_msg) => {
        self.hb = Instant::now();
        ctx.pong(&ping_msg);
      }
      ws::Message::Pong(_) => {
        self.hb = Instant::now();
      }
      ws::Message::Text(text) => {
        ctx.text(format!("{}: {}", self.user_id, text));
      }
      ws::Message::Binary(bin) => ctx.binary(bin),
      ws::Message::Close(reason) => {
        ctx.close(reason);
        ctx.stop();
      }
      ws::Message::Continuation(_) => {
        ctx.stop();
      }
      ws::Message::Nop => (),
    }
  }
}
