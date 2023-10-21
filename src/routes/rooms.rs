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
use crate::models::{Room, ErrorResponse, UserSocket, GameSocket};

#[derive(Deserialize, Debug)]
struct JoinRoomRequest {
  pub room_id: Option<String>
}

fn user_uuid_from_req(req: &AuthenticatedUser) -> Result<Uuid, ErrorResponse> {
  let user_id = match &req.user_id {
    Some(id) => match Uuid::parse_str(&id) {
      Ok(uuid) => uuid,
      Err(err) => return Err(ErrorResponse::BadGateway(err.to_string())),
    },
    None => Uuid::new_v4(),
  };

  Ok(user_id)
}

fn find_room(room_id: &Option<String>, rooms: &HashMap<Uuid, Addr<Room>>) -> Result<Addr<Room>, ErrorResponse> {
  if let Some(found_room) = room_id {
    if let Ok(room_uuid) = Uuid::from_str(found_room.as_str()) {
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
    Err(ErrorResponse::NotFound("Room not found".to_string()))
  }
}

fn find_or_create_room(room_id: &Option<String>, rooms: &mut HashMap<Uuid, Addr<Room>>) -> Result<Addr<Room>, ErrorResponse> {
  match find_room(room_id, rooms) {
    Ok(found_room) => Ok(found_room),
    Err(ErrorResponse::NotFound(_)) => {
      let new_room = Room::default();
      let new_room_id = new_room.id;
      let new_room_addr = new_room.start();

      rooms.insert(new_room_id, new_room_addr.to_owned());

      Ok(new_room_addr)
    },
    Err(error) => return Err(error)
  }
}

#[get("/ping")]
async fn ping() -> impl Responder {
  HttpResponse::Ok().json(json!({"status": "success", "message": "Pong from ROOMS :D"}))
}

#[get("/game/{room_id}")]
async fn game(req: AuthenticatedUser, room_request: Path<JoinRoomRequest>, data: Data<AppState>, stream: Payload) -> Result<HttpResponse, Error> {
  let join_room = find_or_create_room(&room_request.room_id.to_owned(), &mut data.rooms.lock().unwrap())?;

  // todo: to be moved when I use a real db
  let user_id = match user_uuid_from_req(&req) {
    Ok(uuid) => uuid,
    Err(err) => return Err(Error::from(err)),
  };
  
  let game_socket = GameSocket::new(user_id, join_room);
  let resp = ws::start(game_socket, &req.req, stream);

  Ok(resp?)
}

#[routes]
#[get("")]
#[get("/{room_id}")]
async fn room(req: AuthenticatedUser, room_request: Path<JoinRoomRequest>, data: Data<AppState>, stream: Payload) -> Result<HttpResponse, Error> {
  let join_room = find_or_create_room(&room_request.room_id, &mut data.rooms.lock().unwrap())?;

  // todo: to be moved when I use a real db
  let user_id = match user_uuid_from_req(&req) {
    Ok(uuid) => uuid,
    Err(err) => return Err(Error::from(err)),
  };

  let user_socket = UserSocket::new(user_id, join_room.to_owned());
  let resp = ws::start(user_socket, &req.req, stream);

  Ok(resp?)
}

pub fn routes(conf: &mut web::ServiceConfig) {
  let scope = web::scope("/rooms")
    .service(ping)
    .service(game)
    .service(room);

  conf.service(scope);
}
