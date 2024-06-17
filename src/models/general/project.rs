#[derive(Debug)]
pub struct ProjectSpec {
    pub project_description: Option<String>,
    pub backend_code: Option<String>,
    pub frontend_code: Option<String>
}

impl ProjectSpec {
    pub fn new(
        project_description: Option<String>,
        backend_code: Option<String>,
        frontend_code: Option<String>
    ) -> Self {
        ProjectSpec {
            project_description,
            backend_code,
            frontend_code
        }
    }
}

pub fn project_focus() -> Vec<String> {
    vec![
        "Backend".to_owned(),
        "Frontend".to_owned(),
        "Fullstack".to_owned()
    ]
}

// #[derive(Debug)]
// pub enum ProjectFocus {
//     Backend(String),
//     Frontend(String),
//     Fullstack(String)
// }

// impl ToString for ProjectFocus {
//     fn to_string(&self) -> std::string::String { 
//         match &self {
//             Self::Backend(s) => s.to_owned(),
//             Self::Frontend(s) => s.to_owned(),
//             Self::Fullstack(s) => s.to_owned(),
//         }
//     }
// }

/// Backend language options
pub fn backend_languages() -> Vec<String> {
    vec![
        "Rust + Axum".to_owned(),
        "Python + Flask".to_owned(),
        "Java + Spring Boot".to_owned(),
        "JavaScript + Express".to_owned(),
        "JavaScript + NestJs".to_owned(),
        "TypeScript + Express".to_owned(),
        "TypeScript + NextJs".to_owned()
    ]
}

/// Frontend language options
pub fn frontend_language() -> Vec<String> {
    vec![
        "JavaScript + React".to_owned(),
        "JavaScript + Svelte".to_owned(),
        "JavaScript + NextJs".to_owned()
    ]
}

#[derive(Debug, Clone)]
pub struct UserInputs {
    pub project_to_build: String,
    pub project_focus: Option<String>,
    pub backend_language: Option<String>,
    pub frontend_language: Option<String>,
    pub llm_model: Option<String>
}


impl UserInputs {
    pub fn new() -> Self {
        UserInputs {
            project_to_build: "".to_string(),
            project_focus: None,
            backend_language: None,
            frontend_language: None,
            llm_model: None
        }
    }
}