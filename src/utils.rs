use std::fs;

pub fn is_file_path_valid(file_path: &str) -> bool {
    let path = std::path::Path::new(file_path);
    return path.exists() && path.is_file();
}

pub fn read_file(file_path: &str) -> std::io::Result<String> {
    fs::read_to_string(file_path)
}

pub fn print_error(error: &str) {
    eprintln!("cwc: {}", error);
}
