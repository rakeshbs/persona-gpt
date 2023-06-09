use crate::openai;
use crate::utils::get_epoch_ms;
use crate::Message;
use std::io::Read;

pub struct Bot {
    pub name: String,
    pub character_description: String,
}

fn summarize_messages(messages: &Vec<Message>) -> String {
    let mut summary = String::new();

    for message in messages.iter().rev() {
        summary.push_str(&message.role);
        summary.push_str(": ");
        summary.push_str(&message.content);
        summary.push_str("\n");
    }
    return summary;
}

use std::fs;
use std::path::Path;

fn check_and_create_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = "./data";

    if !Path::new(dir_path).exists() {
        fs::create_dir_all(dir_path)?;
        println!("Data directory created: {}", dir_path);
    }

    Ok(())
}

impl Bot {
    pub fn new(name: String, character_description: String) -> Bot {
        Bot {
            name,
            character_description,
        }
    }

    // read from character.txt file and set character_description and name and create a new bot
    pub fn from_character_file() -> Bot {
        let mut file = std::fs::File::open("./character.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let character_description = contents;
        check_and_create_directory().unwrap();
        Bot::new("AI".to_string(), character_description)
    }

    pub async fn get_response_for_message(
        &self,
        message: &Message,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let context_history = summarize_messages(&Message::load_context_from_file(1500, message)?);
        let history = summarize_messages(&Message::load_all_from_file(1500)?);
        let context = format!(
            "{}\n{}\n{}",
            self.character_description, context_history, history
        );
        let string_to_send = format!("{}: {}", message.role, message.content);
        let response = openai::stream_response(&string_to_send, &context).await?;
        let mut ai_message = Message::new(
            self.name.to_string(),
            response.clone(),
            get_epoch_ms().to_string(),
        );
        ai_message.get_embedding().await;
        message.save_to_file().await;
        ai_message.save_to_file().await;
        return Ok(response);
    }
}
