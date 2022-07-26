mod message;

pub use message::Message; // Re-export it so we don't have to reimport it with 'use' in the parent

pub struct Chatter {
  pub name: String,
  pub messages_sent: u32,
  banned: bool
}

pub fn is_chatter_banned(chatter: &Chatter) -> bool {
  chatter.banned
}

pub fn ban_chatter(chatter: &mut Chatter) {
  chatter.banned = true;
}

impl Chatter {
  pub fn new(name: String) -> Chatter {
    Chatter {
      name,
      messages_sent: 0,
      banned: false
    }
  }

  pub fn send_message(&mut self, message: Message) {
    self.messages_sent += 1;
  }
}