use async_openai::types::CreateEmbeddingRequestArgs;
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
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

    pub fn save_to_file(&self) {
        use std::io::Write;
        let json = serde_json::to_string(&self).unwrap();
        let mut file = std::fs::File::create(format!("{}.json", self.uuid)).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

pub async fn get_text_embedding(text: &String) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-ada-002")
        .input([text])
        .build()?;

    let response = client.embeddings().create(request).await?;
    return Ok(response.data[0].embedding.clone());
}

pub async fn get_response(message: &String) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content("You are Lijo Jose Pellissery. You talk like he would talk.")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(message)
                .build()?,
        ])
        .build()?;

    let response = client.chat().create(request).await?;
    return Ok(response.choices[0].message.content.clone());
}
