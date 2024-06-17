use dotenv::dotenv;
use std::fs;

/// Writes LLM generated code to specified filepath
pub fn write_to_file(content: &str, filepath: &str) {
    fs::write(filepath, content)
        .expect(format!("Failed to write to filepath: {}", filepath).as_str());
}

#[cfg(test)]
mod tests {

    use std::env;

    use super::*;
    use crate::{models::general::llm::{LLMModel, Message}, utils::llm_requests::llm_request};

    #[tokio::test]
    async fn test_write_to_file() {
        dotenv().ok();
        let msg = Message {
            role: "user".to_string(),
            content: "Can you write a Rust application that prints a poem?".to_string()
        };
        let content: String = llm_request(vec![msg], None).await.unwrap();
        let filepath: String = env::var("CODE_OUTPUT_PATH").expect("could not find CODE_OUTPUT_PATH");        
        write_to_file(content.as_str(), format!("{}main.rs", filepath).as_str());
    }
}