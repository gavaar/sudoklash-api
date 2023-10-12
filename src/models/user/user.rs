use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
  pub id: Option<String>,
  pub name: String,
  pub email: String,
  pub photo: String,
  pub provider: String,
  pub createdAt: Option<DateTime<Utc>>,
  pub updatedAt: Option<DateTime<Utc>>,
}
