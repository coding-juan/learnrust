mod chatter;
mod actions;

use chatter::{Chatter, Message}; // No need to use self here as 'mod chatter' already imports the module into scope

fn main() {
    let mut user_one = Chatter::new("Jon".to_string());
    let mut user_two = Chatter::new("Ty".to_string());

    let first_message = Message::new("Hello Ty!".to_string());
    user_one.send_message(first_message);

    let second_message = Message::new("Hello Jon!, wup?".to_string());
    user_two.send_message(second_message);

    actions::ban_chatter(&mut user_two);
    let user_status = if chatter::is_chatter_banned(&user_two) {"banned"} else {"active"};
    println!("User {}, is {}!", user_two.name, user_status);
}
