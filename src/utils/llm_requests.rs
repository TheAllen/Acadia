#![allow(dead_code)]
use crossterm::style::Color;
use dotenv::dotenv;
use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use serde::de::DeserializeOwned;
use std::env;

use crate::{agents::base::agent_traits::AgentState, models::general::llm::{LLMModel, LLMRequestBody, LLMResponse, Message, OpenAIResponse}, utils::command_line::LogMessage};

/// Main way to interface with local LLM
pub async fn llm_request(messages: Vec<Message>, model: Option<LLMModel>) -> Result<String, Box<dyn std::error::Error + Send>> {
    // load .env file 
    dotenv().ok();
    
    let url: String;

    // Create the headers
    let mut header_map = HeaderMap::new();

    match &model {
        Some(LLMModel::GPT4o(model_info)) => {
            println!("{} selected!", model_info);

            url = env::var("OPEN_AI_URL").expect("GPT4o variable not found in .env file");
            
            // OPENAI creds
            let key: String = env::var("OPEN_AI_KEY").expect("Could not find OPENAI key from .env file");
            let org: String = env::var("OPEN_AI_ORG").expect("Could not find OPENAI org from .env file");

            header_map.insert("authorization",
                HeaderValue::from_str(&format!("Bearer {}", key).as_str())
                .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })? // propagate error up if any encountered
            );

            header_map.insert("OpenAI-Organization",
                HeaderValue::from_str(&org)
                .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
            );
        },
        Some(LLMModel::Llama3(model_info)) => {
            println!("{} selected!", model_info);

            url = env::var("LLM_URL").expect("LLM_URL variable not found in .env file");

            header_map.insert(
                "Content-Type",
                HeaderValue::from_str("application/json")
                .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
            );
        },
        None => {
            println!("No LLM model specified. Llama3 will be used as default");

            url = env::var("LLM_URL").expect("LLM_URL variable not found in .env file");

            header_map.insert(
                "Content-Type",
                HeaderValue::from_str("application/json")
                .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
            );
        }
    }
    
    let client: Client = Client::builder()
        .default_headers(header_map)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    
    // let llm_req_body: LLMPayloadType;
    // let llm_req_body = LLMPayloadType::Llama3(LLMRequestBody::new(prompt, 50, None));
    let generated_text: String;
    let res: LLMResponse;
    let llm_req_body: LLMRequestBody;

    match &model {
        Some(LLMModel::GPT4o(_)) => {   
            let model = env::var("LLM_MODEL").expect("LLM_MODEL not found in .env file");
            llm_req_body = LLMRequestBody::new(messages, model);
            // println!("{:?}", openai_req_body);

            let res: OpenAIResponse = client
                .post(&url)
                .json(&llm_req_body)
                .send()
                .await.map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
                .json()
                .await.map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;
            generated_text = res.choices[0].message.content.clone();
        },
        _ => {     
            llm_req_body = LLMRequestBody::new(messages, "Llama3".to_owned());

            res = client
                .post(&url)
                .json(&llm_req_body)
                .send()
                .await.map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
                .json()
                .await.map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;
        
            generated_text = res.generated_text.to_owned();
        }
    }

    Ok(generated_text)
}


/// Converts function structure to static string reference for request payload
fn api_instruction_wrapper(func: fn(&str) -> &'static str, user_input: &str) -> Message {
    // Only works with the procedural macro to convert function into string reference
    let ai_function: &str = func(user_input);

    // LLM instructions
    let content: String = format!(
        "FUNCTION: {}
        INSTRUCTION: You are a function printer, you ONLY print the results of functions
        and NOTHING else. No commentary. Here is the input of the function: {}.
        Print out what the function will return.
        ",
        ai_function, user_input
    );

    Message {
        role: "system".to_string(),
        content
    }
}

pub async fn make_llm_request(
    llm_model: LLMModel,
    ai_func: fn(&str) -> &'static str,
    user_req: String,
    agent_position: &str,
    agent_state: &AgentState,
    agent_operation: &str
) -> String {
    
    let req_str: Message = api_instruction_wrapper(ai_func, &user_req);
    LogMessage::Info.print_message(
        &format!("Agent: {} | State: {:?} | Performing: {}", agent_position, agent_state, agent_operation), 
        Color::Rgb { r: 219, g: 255, b: 51 }
    );

    // LLMModel::GPT4o("GPT4o".to_string())
    let detailed_llm_res = llm_request(vec![req_str], Some(llm_model)).await;

    match detailed_llm_res {
        Ok(res) => res,
        Err(e) => {
            panic!("Error occurred when calling LLM service: {}", e);
        }
    }
}

pub async fn make_llm_request_decoded<T: DeserializeOwned>(
    llm_model: LLMModel,
    ai_func: fn(&str) -> &'static str,
    user_req: String,
    agent_position: &str,
    agent_state: &AgentState,
    agent_operation: &str
) -> T {
    let llm_res = make_llm_request(llm_model, ai_func, user_req, agent_position, agent_state, agent_operation).await;
    let res: T = serde_json::from_str(llm_res.as_str()).expect("Could not deserialize LLM response");
    res
}

#[cfg(test)]
mod tests {

    use crate::{ai_functions::ai_functions::convert_user_input_to_goal, function_string};

    use super::*;

    #[tokio::test]
    async fn test_llm_request() {
        let msg = Message {
            role: "user".to_string(),
            content: "This is just a test. Can you provide the shortest response possible?".to_string()
        };

        let res = llm_request(vec![msg], Some(LLMModel::Llama3("Llama 3 from Meta".to_string()))).await;

        match res {
            Ok(r) => {
                dbg!(r);
                ()   
            },
            Err(_) => panic!("Something went wrong with test_llm_request")
        }
    }

    #[tokio::test]
    async fn test_gpt4() {
        let msg = Message {
            role: "user".to_string(),
            content: "This is just a test. Can you provide the shortest response possible?".to_string()
        };

        let res = llm_request(vec![msg], Some(LLMModel::GPT4o("GPT4o".to_string()))).await.unwrap();

        dbg!(res);
    }

    #[tokio::test]
    async fn test_elaborate_llm_request() {
        let res = make_llm_request(
            LLMModel::Llama3("Llama3".to_string()),
            convert_user_input_to_goal,
            "Build a super simple todo app".to_string(),
            "Project Manager",
            &AgentState::Discovery,
            function_string!(convert_user_input_to_goal)
        ).await;

        dbg!(res);
    }
}