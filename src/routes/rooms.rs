use std::collections::HashMap;
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

fn find_or_create_room(room_id: &Option<String>, rooms: &mut HashMap<Uuid, Addr<Room>>) -> Result<Addr<Room>, ErrorResponse> {
  if let Some(room) = room_id {
    if let Ok(room_uuid) = Uuid::from_str(room.as_str()) {
      if let Some(found_room) = rooms.get(&room_uuid) {
        Ok(found_room.to_owned())
      }
      else {
        Err(ErrorResponse::NotFound("Room not found".to_string()))
      }
    } else {
      Err(ErrorResponse::Unauthorized(String::from("Could not parse room_id")))
    }
  } else {
    let new_room = Room::default();
    let new_room_id = new_room.id;
    println!("room_id: {:#?}", &new_room_id);
    let new_room_addr = new_room.start();
    rooms.insert(new_room_id, new_room_addr.to_owned());
    Ok(new_room_addr)
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
  let join_room = find_or_create_room(&room.room_id, &mut data.rooms.lock().unwrap())?;

  let user_id = match req.user_id {
    Some(id) => match Uuid::parse_str(&id) {
      Ok(uuid) => uuid,
      Err(err) => return Err(Error::from(ErrorResponse::BadGateway(err.to_string()))),
    },
    None => Uuid::new_v4(),
  };

  let user_socket = UserSocket::new(user_id, join_room.to_owned());
  let resp = ws::start(user_socket, &req.req, stream);

  Ok(resp?)
}

pub fn routes(conf: &mut web::ServiceConfig) {
  let scope = web::scope("/rooms")
    .service(ping)
    .service(join);

  conf.service(scope);
}
