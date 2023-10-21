use actix_web::{
  cookie::{time::Duration as ActixWebDuration, Cookie},
  get, HttpResponse, Responder, web, HttpRequest,
};

use crate::{guards::AuthenticationGuard, models::User, db::AppState};

mod oauth;

#[get("/temp_user")]
async fn temp_user(_: HttpRequest, data: web::Data<AppState>) -> impl Responder {
  let db_data = &mut data.users.lock().unwrap();
  let user = User::temp();
  let token = oauth::access_token::build(&user.id, &data.env);
  match token {
    Err(e) => e.throw(),
    Ok(token) => {
      db_data.push(user);
      
      println!("token: {:?}, db: {:#?}", token, db_data);

      HttpResponse::Found()
        .append_header(("token", token))
        .finish()
    }
  }
}

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
    .service(temp_user)
    .service(logout)
    .service(oauth::google);

  conf.service(scope);
}
