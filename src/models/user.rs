use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
  pub id: String,
  pub name: String,
  pub email: String,
  pub photo: String,
  pub provider: String,
  pub createdAt: DateTime<Utc>,
  pub updatedAt: DateTime<Utc>,
}
impl User {
  pub fn temp() -> User {
    User {
      id: Uuid::new_v4().to_string(),
      name: String::from("Hippo"),
      email: String::new(),
      photo: String::new(),
      provider: String::from("Temp"),
      createdAt: Utc::now(),
      updatedAt: Utc::now(),
    }
  }
}
