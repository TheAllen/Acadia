use std::io::{stdin, stdout};

use crossterm::{style::{Color, ResetColor, SetForegroundColor}, ExecutableCommand};
use dialoguer::Select;

use crate::models::general::{llm::{llm_choices, LLMModel}, project::{backend_languages, frontend_language, project_focus, UserInputs}};

#[derive(Debug)]
pub enum LogMessage {
    Info,
    Testing,
    Error
}

impl LogMessage {

    pub fn print_message(&self, msg: &str, color: Color) {
        let mut stdout = stdout();

        stdout.execute(SetForegroundColor(color));
        println!("{}", msg);
        stdout.execute(ResetColor);
    }
}

fn prompt_user(prompt: &str, options: Option<Vec<String>>) -> String {
    LogMessage::Info.print_message(prompt, Color::Rgb { r: 2, g: 214, b: 242 });
    let mut line = String::new();
    if let Some(opt) = options {
        let selection = Select::new()
            .items(&opt)
            .default(0)
            .interact()
            .unwrap();
        line = opt[selection].to_owned();
    } else {
        stdin().read_line(&mut line).unwrap();
    }
    LogMessage::Info.print_message(&line, Color::Rgb { r: 69, g: 191, b: 25 });
    line
}

pub fn project_details() -> UserInputs {
    let mut user_inputs = UserInputs::new();
    // Questions
    // 1. What project are we building?
    // 2. What project area of focus?
    // 3. Choose backend language: (Conditional)
    // 4. Choose frontend_language: (Conditional)
    // 5. Which LLM model do you want to build this project?
    let question_one = prompt_user("What project are we building?", None);
    user_inputs.project_to_build = question_one;

    let question_two = prompt_user(
        "What is the project area of focus", 
        Some(project_focus())
    );
    user_inputs.project_focus = Some(question_two.clone());

    match question_two.as_str() {
        "Backend" => {
            let backend_lang = prompt_user("What backend language do you prefer to use?", Some(backend_languages()));
            user_inputs.backend_language = Some(backend_lang);
        },
        "Frontend" => {
            let frontend_lang = prompt_user("What frontend language do you prefer to use?", Some(frontend_language()));
            user_inputs.frontend_language = Some(frontend_lang);
        },
        "Fullstack" => {
            let backend_lang = prompt_user("What backend language do you prefer to use?", Some(backend_languages()));
            user_inputs.backend_language = Some(backend_lang);

            let frontend_lang = prompt_user("What frontend language do you prefer to use?", Some(frontend_language()));
            user_inputs.frontend_language = Some(frontend_lang);
        },
        _ => todo!()
    }

    let llm_choice = prompt_user("Which LLM model do you want to build this project?", Some(llm_choices()));
    user_inputs.llm_model = Some(llm_choice);

    user_inputs
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_message() {

        LogMessage::Info.print_message(
            "This is a log message",
            Color::Rgb { r: 3, g: 252, b: 148 }
        );

        println!("Color should reset");
    }

    #[test]
    fn test_prompt_user() {
        let answer = prompt_user("What project are we building?", None);
        dbg!(answer);
    }

    #[test]
    fn test_prompt_user_options() {
        let answer = prompt_user(
            "What programming language do you want to build your projec in?",
            Some(vec!["Python".to_string(), "Rust".to_string()])
        );
        dbg!(answer);
    }

    #[test]
    fn test_fill_project_details() {
        let project_details = project_details();
        dbg!(project_details);
    }
}
