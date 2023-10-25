use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rand::prelude::*;

const POSSIBLE_NAMES: ([&str; 6], [&str; 6]) = (
  // Emotions
  ["Bored", "Happy", "Sad", "Hungry", "Angry", "Confused"],
  // Animals
  ["Hippo", "Camel", "Penguin", "Doggo", "Cat", "Rhyno"],
);

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
    let mut rng = rand::thread_rng();
    let rng_points: (usize, usize) = (rng.gen_range(0..=5), rng.gen_range(0..=5));

    User {
      id: Uuid::new_v4().to_string(),
      name: String::from("Hippo"),
      email: String::new(),
      photo: String::new(),
      provider: format!("{} {}", POSSIBLE_NAMES.0[rng_points.0], POSSIBLE_NAMES.1[rng_points.1]),
      createdAt: Utc::now(),
      updatedAt: Utc::now(),
    }
  }
}
