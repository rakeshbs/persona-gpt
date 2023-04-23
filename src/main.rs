mod bot;
mod message;
mod openai;
mod utils;

use bot::Bot;
use message::Message;
use std::io;
use std::io::Write;
use utils::get_epoch_ms;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = Bot::from_character_file();
    loop {
        let mut message_user = String::new();
        print!("USER: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut message_user).unwrap();
        message_user = message_user.trim().to_string();
        if message_user.eq("/quit") {
            break;
        }
        let mut message = Message::new(
            "USER".to_string(),
            message_user.to_string(),
            get_epoch_ms().to_string(),
        );
        message.get_embedding().await;

        print!("\n{}: ", bot.name);
        bot.get_response_for_message(&message).await?;
        println!("\n");
    }
    return Ok(());
}
