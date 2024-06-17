use std::sync::Arc;

use agents::base::development_workflow::ProjectWorkflow;
use dialoguer::Select;
use models::general::project::{ProjectSpec, UserInputs};
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

    let project_workflow = ProjectWorkflow::new(input_ptr);

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
