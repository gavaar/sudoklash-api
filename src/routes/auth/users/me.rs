use actix_web::{get, web, HttpResponse, Responder};

use crate::models::AppState;
use crate::guards::AuthenticationGuard;

#[get("/users/me")]
async fn users_me(auth_guard: AuthenticationGuard, data: web::Data<AppState>) -> impl Responder {
  let vec = data.db.lock().unwrap();

  let user = vec
    .iter()
    .find(|user| user.id == Some(auth_guard.user_id.to_owned()));

  HttpResponse::Ok().json(user.unwrap())
}
