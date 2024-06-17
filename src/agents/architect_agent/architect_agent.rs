use async_trait::async_trait;
use crossterm::style::Color;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{agents::base::{agent_attributes::AgentAttributes, agent_traits::{AgentState, AgentTraits, AsyncExecuteFunctions}}, models::general::project::{ProjectSpec, UserInputs}, utils::command_line::LogMessage};

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

        while self.attributes.state != AgentState::Completed {

            match self.attributes.state {
                AgentState::Discovery => {
                    let project_spec = project_spec.read().await;
                    println!("Solutions Architect Project Spec: {:?}", project_spec);
                    // TESTING
                    self.attributes.update_agent_state(AgentState::Completed);
                },
                AgentState::Completed => break,
                _ => self.attributes.update_agent_state(AgentState::Completed)
            }
        }
        Ok(())
    }
}