use std::time::Instant;

pub struct Message {
  pub content: String,
  pub state: State,
  pub timestamp: Instant
}

impl Message {
  pub fn new(content: String) -> Message {
    Message {
      content,
      state: Message::get_default_state(),
      timestamp: Instant::now()
    }
  }

  fn get_default_state() -> State {
    State::Local
  }
}

pub enum State {
  Sent,
  Failed,
  Deleted,
  Local
}