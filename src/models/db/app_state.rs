// dbwork: use a real database
use std::sync::{Arc, Mutex};

use crate::{config::Environment, models::User};

pub struct AppState {
  pub db: Arc<Mutex<Vec<User>>>,
  pub env: Environment,
}
impl AppState {
  pub fn init() -> AppState {
    AppState {
      db: Arc::new(Mutex::new(Vec::new())),
      env: Environment::init(),
    }
  }
}
