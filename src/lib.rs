use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "cwc")]
#[command(author = "Ebooth <pauldejeandev@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "A copy of unix command line tool wc", long_about = None)]
pub struct Args {
    #[arg(short = 'c', group = "byte_count")]
    pub bytes: bool,

    #[arg(short = 'l', group = "line_count")]
    pub lines: bool,

    pub file_paths: Vec<String>,
}

pub fn execute_command(args: Args) -> i32 {
    // println!("args: {}", args.file_paths.join(" "));
    let file_paths = args.file_paths;
    if file_paths.is_empty() {
        print_error("Missing files argument");
        return 1;
    }

    let flags = vec![args.bytes, args.lines];
    let count = flags.into_iter().filter(|&flag| flag).count();
    if count > 1 {
        print_error("Only one of -c (bytes) or -l (lines) can be provided.");
        std::process::exit(1);
    }
    if count == 0 {
        print_error("No flag provided. Please provide either -c (bytes) or -l (lines).");
        std::process::exit(1);
    }

    let mut error_code = 0;
    let mut total = 0;

    for path in file_paths.iter() {
        if !is_file_path_valid(path) {
            print_error(&format!("{}: open: No such file", path));
            error_code = 1;
            continue;
        }
        let file_content = match read_file(path) {
            Ok(content) => content,
            Err(e) => {
                print_error(&format!("Error reading file: {}", e));
                eprintln!("Error reading file: {}", e);
                error_code = 1;
                continue;
            }
        };

        if args.bytes {
            let bytes = file_content.len();
            total += bytes;
            println!("{:>8} {}", bytes, path);
        } else if args.lines {
            let lines_number = file_content.matches("\n").count();
            total += lines_number;
            println!("{:>8} {}", lines_number, path);
        }
    }
    if file_paths.len() > 1 {
        println!("{:>8} total", total);
    }
    return error_code;
}

fn is_file_path_valid(file_path: &str) -> bool {
    let path = std::path::Path::new(file_path);
    return path.exists() && path.is_file();
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    fs::read_to_string(file_path)
}

fn print_error(error: &str) {
    eprintln!("cwc: {}", error);
}
