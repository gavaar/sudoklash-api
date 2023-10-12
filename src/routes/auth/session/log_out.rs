use actix_web::{
  cookie::{time::Duration as ActixWebDuration, Cookie},
  get, HttpResponse, Responder,
};

use crate::guards::AuthenticationGuard;

#[get("/auth/logout")]
pub async fn logout(_: AuthenticationGuard) -> impl Responder {
  let cookie = Cookie::build("token", "")
    .path("/")
    .max_age(ActixWebDuration::new(-1, 0))
    .http_only(true)
    .finish();

  HttpResponse::Ok()
    .cookie(cookie)
    .json(serde_json::json!({"status": "success"}))
}
