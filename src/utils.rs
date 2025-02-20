use std::fs;
use std::io::{stdin, Read};

pub fn is_file_path_valid(file_path: &str) -> bool {
    let path = std::path::Path::new(file_path);
    return path.exists() && path.is_file();
}

pub fn read_file(file_path: &str) -> std::io::Result<String> {
    fs::read_to_string(file_path)
}

pub fn read_from_stdin() -> Result<String, Box<dyn std::error::Error>> {
    let mut content = Vec::new();
    stdin().lock().read_to_end(&mut content)?;
    let input = String::from_utf8(content)?;
    Ok(input)
}

pub fn print_error(error: &str) {
    eprintln!("cwc: {}", error);
}
