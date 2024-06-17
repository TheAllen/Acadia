use std::{sync::Arc, thread};
use tokio::{sync::RwLock, task, try_join};

use crossterm::style::Color;

use crate::{models::general::project::{ProjectSpec, UserInputs}, utils::command_line::LogMessage};

use super::agent_traits::AsyncExecuteFunctions;

/// This object intends to encapsulate all working agents and facilatate the workflow
/// of the project.
pub struct ProjectWorkflow {
    pub agents: Vec<Box<dyn AsyncExecuteFunctions>>,
    pub project_spec: Arc<RwLock<ProjectSpec>>,
    pub user_input: Box<Arc<UserInputs>>
}

impl ProjectWorkflow {
    pub fn new(user_input: Box<Arc<UserInputs>>) -> Self {
        ProjectWorkflow {
            agents: Vec::new(),
            project_spec: Arc::new(RwLock::new(ProjectSpec::new(None, None, None))),
            user_input
        }
    }

    pub fn add_agent(&mut self, agent: Box<dyn AsyncExecuteFunctions>) {
        self.agents.push(agent);
    }

    pub async fn initiate_workflow(&mut self, project_spec: &mut Arc<RwLock<ProjectSpec>>) {
        LogMessage::Info.print_message(
            "Initiating project workflow", 
            Color::Rgb { r: 19, g: 214, b: 185}
        );
        for agent in &mut self.agents {
            let _ = agent.execute_workflow(
                &mut self.project_spec, 
                self.user_input.clone()
            ).await;
        }

    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};
    use tokio::{sync::RwLock, time::sleep};
    use crate::{agents::{architect_agent::architect_agent::ArchitectAgent, base::{agent_attributes::AgentAttributes, agent_traits::AgentState}, manager_agent::manager_agent::ManagerAgent}, models::general::project::ProjectSpec, utils::command_line::project_details};

    use super::*;

    #[tokio::test]
    async fn test_initiate_workflow() {
        // TODO - refactor
        let user_input: UserInputs = project_details();
        let input_ptr: Box<Arc<UserInputs>> = Box::new(Arc::new(user_input));
        let mut project_spec = Arc::new(RwLock::new(ProjectSpec::new(None, None, None)));

        let manager_agent = ManagerAgent::new();
        let architect_agent = ArchitectAgent::new();
        let mut project_workflow = ProjectWorkflow::new(input_ptr.clone());
        project_workflow.add_agent(Box::new(manager_agent));
        project_workflow.add_agent(Box::new(architect_agent));
        project_workflow.initiate_workflow(&mut project_spec).await;

    }
}