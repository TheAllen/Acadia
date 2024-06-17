use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::general::project::{ProjectSpec, UserInputs};

#[derive(Debug, PartialEq, Clone)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Completed
}

pub trait AgentTraits {
    fn update_agent_state(&mut self, new_state: AgentState);
}

#[async_trait]
pub trait AsyncExecuteFunctions {

    async fn execute_workflow(
        &mut self, 
        project_spec: &mut Arc<RwLock<ProjectSpec>>,
        user_input: Box<Arc<UserInputs>>
    ) -> Result<(), Box<dyn std::error::Error>>; 
}