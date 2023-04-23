use crate::openai::get_text_embedding;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    uuid: String,
    role: String,
    content: String,
    embedding_vector: Option<Vec<f32>>,
    timestamp: String,
}

impl Message {
    pub fn new(
        role: String,
        content: String,
        timestamp: String,
        embedding_vector: Option<Vec<f32>>,
    ) -> Self {
        Message {
            uuid: Uuid::new_v4().to_string(),
            role,
            content,
            timestamp,
            embedding_vector,
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
        let mut file = std::fs::File::create(format!("../data/{}.json", self.uuid)).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn load_from_file(uuid: &String) -> Result<Message, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(format!("../data/{}.json", uuid))?;
        let reader = std::io::BufReader::new(file);
        let message: Message = serde_json::from_reader(reader)?;
        return Ok(message);
    }

    // get all files in the folder and load the messagtes
    pub fn load_all_from_file() -> Result<Vec<Message>, Box<dyn std::error::Error>> {
        let mut messages: Vec<Message> = Vec::new();
        for entry in std::fs::read_dir("../data")? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".json") {
                let message = Message::load_from_file(&file_name.replace(".json", ""))?;
                messages.push(message);
            }
        }
        return Ok(messages);
    }
}
