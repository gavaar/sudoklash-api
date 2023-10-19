use std::str::FromStr;
use actix::{Addr, Actor};
use actix_web::{web, Responder, routes};
use actix_web::{get, web::{Data, Payload, Path}, HttpResponse, Error};
use actix_web_actors::ws;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::db::AppState;
use crate::guards::AuthenticatedUser;
use crate::models::{Room, ErrorResponse, UserSocket};

#[derive(Deserialize, Debug)]
struct JoinRoomRequest {
  pub room_id: Option<String>
}

fn find_or_create_room(room_id: &Option<String>, data: Data<AppState>) -> Result<Addr<Room>, ErrorResponse> {
  if let None = room_id {
    return Ok(Room::default().start());
  }

  let room_uuid = Uuid::from_str(room_id.as_ref().unwrap().as_str())
    .map_err(|_| ErrorResponse::BadGateway(String::from("Could not parse room_id")))?; // todo: change error

  match data.rooms.get(&room_uuid) {
    Some(room) => Ok(room.to_owned()),
    None => Err(ErrorResponse::NotFound("Room not found".to_string())),
  }
}

#[get("/ping")]
async fn ping() -> impl Responder {
  HttpResponse::Ok().json(json!({"status": "success", "message": "Pong from ROOMS :D"}))
}

#[routes]
#[get("/join")]
#[get("/join/{room_id}")]
async fn join(req: AuthenticatedUser, room: Path<JoinRoomRequest>, data: Data<AppState>, stream: Payload) -> Result<HttpResponse, Error> {
  let join_room = find_or_create_room(&room.room_id, data)?;
  let user_id = match req.user_id {
    Some(id) => match Uuid::parse_str(&id) {
      Ok(uuid) => uuid,
      Err(err) => return Err(Error::from(ErrorResponse::BadGateway(err.to_string()))),
    },
    None => Uuid::new_v4(),
  };

  let user_socket = UserSocket::new(user_id, join_room);
  let resp = ws::start(user_socket, &req.req, stream);

  Ok(resp?)
}

pub fn routes(conf: &mut web::ServiceConfig) {
  let scope = web::scope("/rooms")
    .service(ping)
    .service(join);

  conf.service(scope);
}
