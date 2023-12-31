// create a message that will be sent to openai
// {role: , message: }

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ChatCompletion {
    pub model: String,
    pub messages: Vec<Message>,
    pub temperature: f32,
}

#[derive(Debug, Deserialize)]
pub struct ApiMessage {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiChoice {
    pub message: ApiMessage,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub choices: Vec<ApiChoice>,
}
