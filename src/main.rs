use std::sync::Arc;

use agents::{architect_agent::architect_agent::ArchitectAgent, backend_agent::backend_agent::BackendAgent, base::development_workflow::ProjectWorkflow, manager_agent::manager_agent::ManagerAgent};
use dialoguer::Select;
use models::general::project::{ProjectSpec, UserInputs};
use tokio::sync::RwLock;
use utils::command_line::project_details;


mod ai_functions;
mod agents;
mod macros;
#[macro_use]
mod models;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send>>{
    println!("Welcome to Acadia - an AI engineering tool!");
    
    let user_input: UserInputs = project_details();
    let input_ptr: Box<Arc<UserInputs>> = Box::new(Arc::new(user_input));

    let mut project_workflow = ProjectWorkflow::new(input_ptr);
    let mut project_spec = Arc::new(RwLock::new(ProjectSpec::new()));
    project_workflow.add_agent(Box::new(ManagerAgent::new()));
    project_workflow.add_agent(Box::new(ArchitectAgent::new()));
    project_workflow.add_agent(Box::new(BackendAgent::new()));

    project_workflow.initiate_workflow(&mut project_spec).await;

    // let project_spec = tokio::sync::RwLock::with_max_readers(Box::new(ProjectSpec {
    //     project_description: Some("A simple restaurant application".to_string()),
    //     frontend_code: None,
    //     backend_code: None
    // }), 5);
    // {
    //     println!("{:?}", *project_spec.read().await);
    // }
    // println!("{:?}", *project_spec.read().await);

    Ok(())
}
