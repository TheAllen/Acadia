use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub enum LLMModel {
    Llama3(String),
    GPT4o(String)
}

pub fn llm_choices() -> Vec<String> {
    vec![
        "GPT-4o".to_string(),
        "Llama3".to_string()
    ]
}


// #[derive(Debug, Serialize)]
// pub struct LLMRequestBody {
//     prompt: String,
//     max_length: u32,
//     model: Option<LLMModel>
// }

// impl LLMRequestBody {
//     pub fn new(prompt: String, max_length: u32, model: Option<LLMModel>) -> Self {
//         LLMRequestBody {
//             prompt,
//             max_length,
//             model
//         }
//     }
// }

#[derive(Debug, Deserialize)]
pub struct LLMResponse {
    pub generated_text: String
}

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String
}

#[derive(Serialize, Debug)]
pub struct LLMRequestBody {
    pub model: String,
    pub messages: Vec<Message>
}

impl LLMRequestBody {
    pub fn new(messages: Vec<Message>, model: String) -> Self {
        LLMRequestBody {
            model,
            messages
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenAIAPIMessage {
    pub content: String
}

#[derive(Debug, Deserialize)]
pub struct OpenAIAPIChoice {
    pub message: OpenAIAPIMessage
}


#[derive(Debug, Deserialize)]
pub struct OpenAIResponse {
    pub choices: Vec<OpenAIAPIChoice>
}
