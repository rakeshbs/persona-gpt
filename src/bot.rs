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

impl Bot {
    pub fn new(name: String, character_description: String) -> Bot {
        Bot {
            name,
            character_description,
        }
    }

    // read from character.txt file and set character_description and name and create a new bot
    pub fn from_character_file() -> Bot {
        let mut file = std::fs::File::open("../data/character.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let character_description = contents;
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
        let response = openai::get_response(&message.content, &context).await?;
        let ai_message = Message::new(
            self.name.to_string(),
            response.clone(),
            get_epoch_ms().to_string(),
        );
        message.save_to_file().await;
        ai_message.save_to_file().await;
        return Ok(response);
    }
}
