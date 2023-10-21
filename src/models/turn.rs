use actix::Message;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Message, Serialize, Deserialize, Clone)]
#[rtype(result = "()")]
pub struct Turn {
  pub play: String,
  pub user_id: Uuid,
  // (hit, dead)
  #[serde(skip_deserializing)]
  pub result: (u8, u8),
  #[serde(skip_deserializing)]
  pub played_at: DateTime<Utc>,
}
impl Turn {
  pub fn hit_dead_against_selection(&mut self, selection: u16) {
    self.result = (0, 0);
    let selection_chars: Vec<char> = selection.to_string().chars().collect();
    self.play.to_string().char_indices().for_each(|(index, played_char)| {
      match selection_chars.get(index) {
        Some(selection_char) => {
          if selection_char == &played_char { self.result.0 += 1 }
          else if selection_chars.contains(&played_char) { self.result.1 += 1 }
        }
        _ => eprintln!("Something went wrong when checking for the play's result"),
      }
    });
  }
}
