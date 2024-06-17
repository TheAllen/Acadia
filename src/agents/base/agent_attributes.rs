use crossterm::style::Color;

use crate::{models::general::llm::Message, utils::command_line::LogMessage};

use super::agent_traits::{AgentState, AgentTraits};

#[derive(Debug, Clone)]
pub struct AgentAttributes {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub messages: Vec<Message>,
}

impl AgentAttributes {

    pub fn new(objective: String, position: String) -> Self {
        AgentAttributes {
            objective,
            position,
            state: AgentState::Discovery,
            messages: Vec::new()
        }
    }
}

impl AgentTraits for AgentAttributes {

    fn update_agent_state(&mut self, new_state: AgentState) {
        // TODO: log message about state change
        self.state = new_state;
    }
}