use std::{borrow::BorrowMut, sync::Arc, time::Duration};

use async_trait::async_trait;
use crossterm::style::{Attribute, Color};
use tokio::{sync::RwLock, time};
use crate::{agents::base::{agent_attributes::AgentAttributes, agent_traits::{AgentState, AgentTraits, AsyncExecuteFunctions}}, ai_functions::ai_functions::convert_user_input_to_goal, function_string, models::general::{llm::LLMModel, project::{ProjectSpec, UserInputs}}, utils::{command_line::LogMessage, llm_requests::make_llm_request}};

#[derive(Debug, Clone)]
pub struct ManagerAgent {
    pub attributes: AgentAttributes
}

impl ManagerAgent {
    pub fn new() -> Self {
        let attributes = AgentAttributes::new(
            "manage agents that are bulding the application for the end user".to_string(),
            "Project Manager".to_string()
        );
        ManagerAgent {
            attributes,
        }
    }

    /// Step 1. Generate a project description for Solutions Architect agent to interpret
    async fn articulate_project_description(&mut self, user_req: String, llm_model: LLMModel) -> String {
        let res = make_llm_request(
            llm_model,
            convert_user_input_to_goal, 
            user_req, 
            &self.attributes.position, 
            &self.attributes.state,
            function_string!(convert_user_input_to_goal)
        ).await;
        res
    }
}

// TODO: Create workflow functions

#[async_trait]
impl AsyncExecuteFunctions for ManagerAgent {

    async fn execute_workflow(
        &mut self,
        project_spec: &mut Arc<RwLock<ProjectSpec>>,
        user_input: Box<Arc<UserInputs>>
    ) -> Result<(), Box<dyn std::error::Error>>{
        LogMessage::Info.print_message(
            "Manager agent beginning workflow...", 
            Color::Rgb { r: 19, g: 214, b: 185 }
        );
        // TODO - remove below (TESTING ONLY)
        // println!("{:?}", self);
        // println!("{:p}", &*user_input);
        let user_input = *user_input;
        while self.attributes.state != AgentState::Completed {

            match self.attributes.state {
                AgentState::Discovery => {
                    // Make request to LLM and articulate project description
                    let project_description = self.articulate_project_description(user_input.project_to_build.clone(), LLMModel::Llama3("Llama3".to_string())).await;
                    if let Ok(mut spec) = project_spec.try_write() {
                        spec.project_description = Some(project_description.clone());
                    } else {
                        panic!("Failed to acquire write lock on project_spec");
                    };
                    self.attributes.update_agent_state(AgentState::Completed)
                },
                AgentState::Completed => break,
                _ => self.attributes.update_agent_state(AgentState::Completed)
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::ai_functions::ai_functions::convert_user_input_to_goal;

    use super::*;

    #[test]
    fn create_manager_agent() {
        todo!()
    }

    #[test]
    fn test_project_manager_ai_function() {
        let ai_func_str = convert_user_input_to_goal("Create a budget management app");
        dbg!(ai_func_str);
    }

}