use serde::Deserialize;

#[derive(Deserialize)]
pub struct GoogleUserResult {
  pub id: String,
  pub email: String,
  pub verified_email: bool,
  pub name: String,
  pub given_name: String,
  pub family_name: String,
  pub picture: String,
}
