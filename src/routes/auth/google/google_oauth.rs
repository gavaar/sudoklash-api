// todo: separate this

use actix_web::{get, web, Responder, cookie::{time::Duration as ActixWebDuration, Cookie}, HttpResponse};
use chrono::{Utc, Duration};
use jsonwebtoken::{EncodingKey, Header, encode};
use uuid::Uuid;

use crate::models::{AppState, ErrorResponse, OAuthResponse, User, QueryCode, TokenClaims};
use super::helpers::{request_access_token, user_info};

async fn token_response_from_code(code: &String, data: &web::Data<AppState>) -> Result<OAuthResponse, ErrorResponse> {
  if code.is_empty() {
    return Err(ErrorResponse::Unauthorized("Code to request token was invalid or empty".to_string()));
  }

  request_access_token(code.as_str(), data).await
}

//https://accounts.google.com/o/oauth2/auth?scope=https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email&response_type=code&access_type=offline&redirect_uri=http://localhost:8000/v1/sessions/oauth/google&client_id=921222346302-33pgvo300556qde30v87ot2gqmeikp87.apps.googleusercontent.com
#[get("/sessions/oauth/google")]
pub async fn google_oauth_handler(query: web::Query<QueryCode>, data: web::Data<AppState>) -> impl Responder {
  let token_response = match token_response_from_code(&query.code, &data).await {
    Ok(token) => token,
    Err(error) => return error.throw(),
  };

  let google_user = match user_info(&token_response.access_token, &token_response.id_token).await {
    Ok(google_user) => google_user,
    Err(error) => return error.throw(),
  };

  let mut vec = data.db.lock().unwrap();
  let email = google_user.email.to_lowercase();
  let user = vec.iter_mut().find(|user| user.email == email);

  let user_id: String;

  if user.is_some() {
    let user = user.unwrap();
    user_id = user.id.to_owned().unwrap();
    user.email = email.to_owned();
    user.photo = google_user.picture;
    user.updatedAt = Some(Utc::now());
  } else {
    let datetime = Utc::now();
    let id = Uuid::new_v4();
    user_id = id.to_owned().to_string();
    let user_data = User {
      id: Some(id.to_string()),
      name: google_user.name,
      email,
      provider: "Google".to_string(),
      photo: google_user.picture,
      createdAt: Some(datetime),
      updatedAt: Some(datetime),
    };

    vec.push(user_data.to_owned());
  }

  let jwt_secret = data.env.jwt_secret.to_owned();
  let now = Utc::now();
  let iat = now.timestamp() as usize;
  let exp = (now + Duration::minutes(data.env.jwt_max_age)).timestamp() as usize;
  let claims: TokenClaims = TokenClaims {
    sub: user_id,
    exp,
    iat,
  };

  let token = encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(jwt_secret.as_ref()),
  )
  .unwrap();

  let cookie = Cookie::build("token", token)
    .path("/")
    .max_age(ActixWebDuration::new(60 * data.env.jwt_max_age, 0))
    .http_only(true)
    .finish();

  let mut response = HttpResponse::Found();
  response.cookie(cookie);
  response.finish()
}
