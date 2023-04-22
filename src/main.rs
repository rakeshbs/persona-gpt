mod chat;
use crate::chat::Message;
use std::io;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut message_user = String::new();
        print!("USER: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut message_user).unwrap();
        message_user = message_user.trim().to_string();
        if message_user.eq("/quit") {
            break;
        }
        // unix time stamp

        let mut message = Message::new(
            "USER".to_string(),
            message_user.to_string(),
            get_epoch_ms().to_string(),
            None,
        );

        //messageeget_embedding().await;

        let result = chat::get_response(&message_user).await;
        match result {
            Ok(response) => println!("\nAI: {}\n", response),
            Err(e) => return Err(e),
        }
    }
    return Ok(());
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
