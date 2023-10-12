use serde::Deserialize;

#[derive(Deserialize)]
pub struct OAuthResponse {
  pub access_token: String,
  pub id_token: String,
}
