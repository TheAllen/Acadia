use async_trait::async_trait;
use crossterm::style::Color;
use std::{sync::Arc, time::Duration};

use tokio::sync::RwLock;

use crate::{agents::base::{agent_attributes::AgentAttributes, agent_traits::{AgentState, AgentTraits, AsyncExecuteFunctions}}, ai_functions::ai_functions::{print_project_scope, print_site_urls}, function_string, models::general::{llm::LLMModel, project::{ProjectScope, ProjectSpec, UserInputs}}, utils::{command_line::LogMessage, helper::check_url_status_code, llm_requests::make_llm_request_decoded}};

#[derive(Debug, Clone)]
pub struct ArchitectAgent {
    attributes: AgentAttributes
}

impl ArchitectAgent {

    pub fn new() -> Self {
        let attributes = AgentAttributes::new(
            "Gathers information and design solutions for website development".to_string(),
            "Solutions Architect".to_string()
        );
        ArchitectAgent {
            attributes
        }
    }

    async fn get_project_scope(&mut self, project_spec: &mut Arc<RwLock<ProjectSpec>>) {
        
        let project_description = format!("{:?}", project_spec.read().await.project_description);

        let project_scope: ProjectScope = make_llm_request_decoded::<ProjectScope>(
            LLMModel::GPT4o("GPT4o".to_owned()), 
            print_project_scope, 
            project_description, 
            &self.attributes.position, 
            &self.attributes.state, 
            function_string!(print_project_scope)
        ).await;

        if let Ok(mut proj_spec) = project_spec.try_write() {
            proj_spec.project_scope = Some(project_scope);
        } else {
            LogMessage::Error.print_message("Unable to write project scope to Project Spec", Color::Red);
            panic!("Exiting Architect agent");
        }
        // project_spec.write().await.project_scope = Some(project_scope);
    }

    async fn get_external_urls(&mut self, project_spec: &mut Arc<RwLock<ProjectSpec>>) -> Vec<String> {
        
        let project_description = project_spec.read().await.project_description.clone().expect("Project description not found");

        let external_urls: Vec<String> = make_llm_request_decoded(
            LLMModel::GPT4o("GPT4o".to_owned()), 
            print_site_urls, 
            project_description, 
            &self.attributes.position, 
            &self.attributes.state, 
            function_string!(print_site_urls)
        ).await;

        external_urls

    }

    async fn test_external_urls(&mut self, external_urls: &Vec<String>) -> Vec<String> {
        let mut valid_external_urls = Vec::<String>::new();

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("Could not create reqwest Client");

        for url in external_urls.iter() {
            if check_url_status_code(&client, url).await.unwrap() == 200 {
                valid_external_urls.push(url.to_owned());
            } else {
                LogMessage::Error.print_message(format!("External URL error: {}", url).as_str(), Color::Red);
            }
        }

        valid_external_urls
    }
}

#[async_trait]
impl AsyncExecuteFunctions for ArchitectAgent {

    async fn execute_workflow(
        &mut self,
        project_spec: &mut Arc<RwLock<ProjectSpec>>,
        user_input: Box<Arc<UserInputs>>
    ) -> Result<(), Box<dyn std::error::Error>> {
        LogMessage::Info.print_message(
            "Solutions Architect beginning workflow...", 
            Color::Rgb { r: 19, g: 214, b: 185 }
        );
        let user_input = *user_input;
        let mut external_urls: Vec<String> = vec![];
        while self.attributes.state != AgentState::Completed {

            match self.attributes.state {
                AgentState::Discovery => {
                    
                    self.get_project_scope(project_spec).await;

                    // Check if external urls are required
                    if project_spec.read().await.project_scope.unwrap().is_external_urls_required {
                        self.attributes.update_agent_state(AgentState::Working);
                        continue;
                    }
                    self.attributes.update_agent_state(AgentState::Completed);
                                
                },
                AgentState::Working => {
                    external_urls = self.get_external_urls(project_spec).await;

                    self.attributes.update_agent_state(AgentState::UnitTesting);
                },
                AgentState::UnitTesting => {

                    // Get a vector of valid external urls
                    if external_urls.is_empty() {
                        self.attributes.update_agent_state(AgentState::Completed);
                        continue;
                    }

                    let validated_external_urls = self.test_external_urls(&external_urls).await;
                    println!("These are the validated external URLs: {:?}", validated_external_urls);

                    if let Ok(ref mut proj_spec) = project_spec.try_write() {
                        proj_spec.external_urls = Some(validated_external_urls);
                    } else {
                        LogMessage::Error.print_message("Unable to write external urls to Project Spec", Color::Red);
                        panic!("Exiting Architect agent");
                    }
                    println!("This is the project spec after Solutions Architect is done: {:?}", project_spec.read().await);
                    self.attributes.update_agent_state(AgentState::Completed);
                },
                AgentState::Completed => {
                    println!("This is the project spec after Solutions Architect is done: {:?}", project_spec.read().await);
                },
                _ => self.attributes.update_agent_state(AgentState::Completed)
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_architect_agent() {
        let architect = ArchitectAgent::new();
        dbg!(architect);
    }
}