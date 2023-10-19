use std::{sync::{Arc, Mutex}, collections::HashMap};

use actix::Addr;
use uuid::Uuid;

use crate::{
  models::{User, Room},
  environment::Environment,
};

pub struct AppState {
  pub db: Arc<Mutex<Vec<User>>>,
  pub rooms: HashMap<Uuid, Addr<Room>>,
  pub env: Environment,
}
impl AppState {
  pub fn init() -> AppState {
    AppState {
      db: Arc::new(Mutex::new(Vec::new())),
      rooms: HashMap::new(),
      env: Environment::init(),
    }
  }
}
