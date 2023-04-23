use async_openai::types::CreateEmbeddingRequestArgs;
use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};

pub async fn get_text_embedding(text: &String) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-ada-002")
        .input([text])
        .build()?;

    let response = client.embeddings().create(request).await?;
    return Ok(response.data[0].embedding.clone());
}

pub async fn get_response(
    message: &String,
    context: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(context)
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

use futures::StreamExt;
use std::io::{stdout, Write};

pub async fn stream_response(
    message: &String,
    context: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(context)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(message)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content("AI: ")
                .build()?,
        ])
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;
    let mut reply: String = "".to_string();
    let mut lock = stdout().lock();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        reply.push_str(content);
                        write!(lock, "{}", content).unwrap();
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }
    return Ok(reply);
}
