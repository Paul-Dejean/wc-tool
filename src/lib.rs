mod errors;
mod utils;

use clap::Parser;
use errors::ProcessFileError;
use std::fmt;
use utils::{is_file_path_valid, print_error, read_file, read_from_stdin};

#[derive(Parser, Debug)]
#[command(name = "cwc")]
#[command(author = "Ebooth <pauldejeandev@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "A copy of unix command line tool wc", long_about = None)]
pub struct Args {
    #[arg(short = 'c', group = "byte_count")]
    bytes: bool,

    #[arg(short = 'l')]
    lines: bool,

    #[arg(short = 'w')]
    words: bool,

    #[arg(short = 'm', group = "byte_count")]
    chars: bool,

    file_paths: Vec<String>,
}

pub fn execute_command(args: &mut Args) -> i32 {
    let file_paths = &args.file_paths;

    if [args.bytes, args.chars, args.lines, args.words]
        .iter()
        .all(|&x| !x)
    {
        args.bytes = true;
        args.lines = true;
        args.words = true;
    }

    if file_paths.is_empty() {
        let content = read_from_stdin();
        match content {
            Ok(c) => {
                let wc_result = process_content("", &c, &args);
                println!("{}", wc_result);
                return 0;
            }
            Err(e) => {
                print_error(&format!("Error reading from stdin: {}", e));
                return 1;
            }
        }
    }

    if file_paths.len() == 1 {
        let result = process_file(&file_paths[0], &args);
        match result {
            Ok(r) => {
                println!("{}", r);
                return 0;
            }
            Err(_) => return 1,
        }
    }

    let mut error_code = 0;
    let mut total_result = WcResult::new("total".to_string());
    for path in file_paths.iter() {
        let result = process_file(path, &args);
        match result {
            Ok(r) => {
                println!("{}", r);
                total_result.add(&r);
            }
            Err(_) => {
                error_code = 1;
            }
        }
    }
    println!("{}", total_result);
    return error_code;
}

#[derive(Debug)]
struct WcResult {
    label: String,
    bytes: Option<usize>,
    lines: Option<usize>,
    words: Option<usize>,
    chars: Option<usize>,
}

impl WcResult {
    fn new(label: String) -> WcResult {
        WcResult {
            label,
            bytes: None,
            lines: None,
            words: None,
            chars: None,
        }
    }

    fn add(&mut self, other: &Self) {
        if let Some(b) = other.bytes {
            self.bytes = Some(b + self.bytes.unwrap_or(0));
        }
        if let Some(c) = other.chars {
            self.chars = Some(c + self.chars.unwrap_or(0));
        }
        if let Some(w) = other.words {
            self.words = Some(w + self.words.unwrap_or(0));
        }
        if let Some(l) = other.lines {
            self.lines = Some(l + self.lines.unwrap_or(0));
        }
    }
}

impl fmt::Display for WcResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = [self.lines, self.words, self.bytes, self.chars]
            .iter()
            .filter_map(|&x| x)
            .map(|x| format!("{x:>7}"))
            .collect::<Vec<_>>()
            .join(" ");
        write!(f, "{} {}", fields, self.label)
    }
}

fn process_file(file_path: &str, args: &Args) -> Result<WcResult, ProcessFileError> {
    if !is_file_path_valid(file_path) {
        print_error(&format!("{}: open: No such file", file_path));
        return Err(ProcessFileError::FileNotFound(file_path.to_string()));
    }
    let file_content = match read_file(file_path) {
        Ok(content) => content,
        Err(e) => {
            print_error(&format!("Error reading file: {}", e));
            return Err(ProcessFileError::IoError(e));
        }
    };
    return Ok(process_content(file_path, &file_content, args));
}

fn process_content(file_path: &str, content: &str, args: &Args) -> WcResult {
    let mut wc_result = WcResult::new(file_path.to_string());
    if args.bytes {
        wc_result.bytes = Some(content.len());
    };

    if args.lines {
        wc_result.lines = Some(content.matches("\n").count());
    };

    if args.words {
        wc_result.words = Some(content.split_whitespace().count());
    }
    if args.chars {
        wc_result.chars = Some(content.chars().count());
    }

    return wc_result;
}
