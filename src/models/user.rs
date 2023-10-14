use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
