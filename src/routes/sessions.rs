use actix_web::{
  cookie::{time::Duration as ActixWebDuration, Cookie},
  get, HttpResponse, Responder, web,
};

use crate::guards::AuthenticationGuard;

mod oauth;

#[get("/logout")]
async fn logout(_: AuthenticationGuard) -> impl Responder {
  let cookie = Cookie::build("token", "")
    .path("/")
    .max_age(ActixWebDuration::new(-1, 0))
    .http_only(true)
    .finish();

  HttpResponse::Ok()
    .cookie(cookie)
    .json(serde_json::json!({"status": "success"}))
}

pub fn routes(conf: &mut web::ServiceConfig) {
  let scope = web::scope("/sessions")
    .service(logout)
    .service(oauth::google);

  conf.service(scope);
}
