use crate::utils::read_and_sort_dir;
use crate::{openai::get_text_embedding, utils::token_length};
use serde::{Deserialize, Serialize};

pub fn get_message_similarity(message1: &Message, message2: &Message) -> f32 {
    let vector1 = message1.embedding_vector.as_ref().unwrap();
    let vector2 = message2.embedding_vector.as_ref().unwrap();
    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;
    for i in 0..vector1.len() {
        dot_product += vector1[i] * vector2[i];
        norm1 += vector1[i] * vector1[i];
        norm2 += vector2[i] * vector2[i];
    }
    return dot_product / (norm1 * norm2).sqrt();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
    pub embedding_vector: Option<Vec<f32>>,
    pub timestamp: String,
}

impl Message {
    pub fn new(role: String, content: String, timestamp: String) -> Self {
        Message {
            role,
            content,
            timestamp,
            embedding_vector: None,
        }
    }

    pub async fn get_embedding(&mut self) {
        let embedding_vector = get_text_embedding(&self.content).await;
        if embedding_vector.is_err() {
            println!("Error getting embedding vector: {:?}", embedding_vector);
            return;
        }
        self.embedding_vector = Some(embedding_vector.unwrap());
    }

    pub async fn save_to_file(&self) {
        use std::io::Write;
        let json = serde_json::to_string(&self).unwrap();
        let mut file = std::fs::File::create(format!("../data/{}.json", self.timestamp)).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn load_from_file(uuid: &String) -> Result<Message, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(format!("../data/{}.json", uuid))?;
        let reader = std::io::BufReader::new(file);
        let message: Message = serde_json::from_reader(reader)?;
        return Ok(message);
    }

    // get all files in the folder and load the messagtes
    pub fn load_all_from_file(
        max_tokens: usize,
    ) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
        let mut messages: Vec<Message> = Vec::new();
        let files = read_and_sort_dir("../data")?;
        let mut total_tokens = 0;

        for entry in files {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".json") {
                let message = Message::load_from_file(&file_name.replace(".json", ""))?;
                let tokens = token_length(&message.content);
                total_tokens += tokens;
                if total_tokens > max_tokens {
                    break;
                }
                messages.push(message);
            }
        }
        return Ok(messages);
    }

    pub fn load_context_from_file(
        max_tokens: usize,
        query: &Message,
    ) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
        let mut messages: Vec<Message> = Vec::new();
        let files = read_and_sort_dir("../data")?;
        let mut total_tokens = 0;

        for entry in files {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".json") {
                let message = Message::load_from_file(&file_name.replace(".json", ""))?;
                let tokens = token_length(&message.content);
                if get_message_similarity(&query, &message) > 0.8 {
                    total_tokens += tokens;
                    if total_tokens > max_tokens {
                        break;
                    }
                    messages.push(message);
                }
            }
        }
        return Ok(messages);
    }
}
