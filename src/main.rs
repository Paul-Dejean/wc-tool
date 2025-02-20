use clap::Parser;
use cwc_tool::{execute_command, Args};
use std::process;

fn main() {
    let mut args = Args::parse();
    let error_code = execute_command(&mut args);
    process::exit(error_code);
}
