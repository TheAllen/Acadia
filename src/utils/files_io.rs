use std::fs;

// TODO: Remove
const CODE_TEMPLATE_PATH: &str = "code_templates/";

// Reading data
pub fn read_code_template_contents(language: String) -> String {
    let mut path: String = String::from(CODE_TEMPLATE_PATH);
    match language.as_str() {
        "Rust" | "rust" => path.push_str("rust_axum/main.rs"),
        _ => path.push_str("rust_axum/main.rs")
    }
    println!("{}", path);
    fs::read_to_string(path).expect("Failed to read code template")
}

// Writing data
pub fn write_code_template_contents(contents: &String, language: String) {
    let path = String::from("generated_code/main.rs");
    fs::write(path, contents).expect("Failed to save file");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_write_code_file() {
        let code_file = read_code_template_contents("Rust".to_string());
        write_code_template_contents(&code_file, "Rust".to_string());
        
    }
}