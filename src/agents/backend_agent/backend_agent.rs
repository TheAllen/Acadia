use std::sync::Arc;

use async_trait::async_trait;
use crossterm::style::Color;
use tokio::sync::RwLock;

use crate::{agents::base::{agent_attributes::AgentAttributes, agent_traits::AsyncExecuteFunctions}, ai_functions::ai_functions::print_backend_webserver_code, function_string, models::general::{llm::LLMModel, project::{ProjectSpec, UserInputs}}, utils::{command_line::LogMessage, llm_requests::make_llm_request}};


#[derive(Debug, Clone)]
pub struct BackendAgent {
    pub attributes: AgentAttributes
}

impl BackendAgent {

    pub fn new() -> Self {
        let attributes = AgentAttributes::new(
            "Develops the backend code for webserver and json database".to_owned(),
            "Backend Developer".to_owned()
        );
        BackendAgent {
            attributes
        }
    }

    async fn generate_backend_code(&mut self, project_spec: &mut Arc<RwLock<ProjectSpec>>, preferred_language: Option<String>) {
        if preferred_language == None {
            LogMessage::Error.print_message("No language was selected. Exiting program", Color::Red);
            panic!();
        }
        let project_description = project_spec.as_ref().read().await.project_description.clone().unwrap();
        let extended_description = format!("{} built using {}", project_description, preferred_language.unwrap());

        if let Ok(ref mut proj_spec) = project_spec.try_write() {
            let backend_code = make_llm_request(
                LLMModel::GPT4o("GPT4o".to_owned()), 
                print_backend_webserver_code, 
                extended_description, 
                &self.attributes.position, 
                &self.attributes.state, 
                function_string!(print_backend_webserver_code)
            ).await;
            // proj_spec.backend_code = Some(backend_code);
            println!("{}", backend_code);
        } else {
            panic!("Failed to generate backend code");
        }
    }
}

#[async_trait]
impl AsyncExecuteFunctions for BackendAgent {

    // Apply selected language to the object of the backend agent
    async fn execute_workflow(
        &mut self, 
        project_spec: &mut Arc<RwLock<ProjectSpec>>, 
        user_input: Box<Arc<UserInputs>>
    ) -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }
}

#[cfg(test)]
mod tests {


    use super::*;

    #[tokio::test]
    async fn test_generate_backend_code() {
        let mut project_spec = Arc::new(RwLock::new(ProjectSpec::new()));
        project_spec.write().await.project_description = Some("Build a very simple todo app with just a get and post route".to_string());

        let mut backend_agent = BackendAgent::new();
        backend_agent.generate_backend_code(&mut project_spec, Some("Rust".to_string())).await;
    }
}