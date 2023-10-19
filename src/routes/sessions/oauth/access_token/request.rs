use actix_web::web;
use reqwest::Client;

use crate::{
  models::{OAuthResponse, ErrorResponse},
  db::AppState,
};

pub async fn request(
  authorization_code: &str,
  data: &web::Data<AppState>,
) -> Result<OAuthResponse, ErrorResponse> {
  if authorization_code.is_empty() {
    return Err(ErrorResponse::Unauthorized("Code to request token was invalid or empty".to_string()));
  }

  let redirect_url = data.env.google_oauth_redirect_url.to_owned();
  let client_secret = data.env.google_oauth_client_secret.to_owned();
  let client_id = data.env.google_oauth_client_id.to_owned();
  let root_url = "https://oauth2.googleapis.com/token";
  let client = Client::new();

  let params = [
    ("grant_type", "authorization_code"),
    ("redirect_uri", redirect_url.as_str()),
    ("client_id", client_id.as_str()),
    ("code", authorization_code),
    ("client_secret", client_secret.as_str()),
  ];

  let response = client.post(root_url)
    .form(&params)
    .send()
    .await
    .map_err(|err| ErrorResponse::BadGateway(err.to_string()))?;

  response.json::<OAuthResponse>()
    .await
    .map_err(|_| ErrorResponse::BadGateway("Something went wrong when retrieving access token".to_owned()))
}
