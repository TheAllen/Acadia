use std::fs;

// TODO: Remove
const CODE_TEMPLATE_PATH: &str = "/Users/allenli/Projects/rust/web_template/src/code_template.rs";

// Reading data
pub fn read_code_template_contents(language: String) -> String {
    let path: String = String::from(CODE_TEMPLATE_PATH);
    fs::read_to_string(path).expect("Failed to read code template")
}