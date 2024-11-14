use std::{env, fs, process};
use zhersh_json_parser::{validate_json, JsonParseError};

fn print_help() {
    println!("Usage:");
    println!("  cargo run -- zhersh_json.json   Parses the specified JSON file.");
    println!("  cargo run -- --help        Displays this help information.");
    println!("  cargo run -- --credits     Shows credits for this project.");
}

fn print_credits() {
    println!("JSON Parser Project");
    println!("Author: Maksym Zhersh (AM-3)");
    println!("Repository: https://github.com/Tutt1Frutty/my_json_parser.git");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        process::exit(1);
    }

    match args[1].as_str() {
        "--help" => print_help(),
        "--credits" => print_credits(),
        file_path => match fs::read_to_string(file_path) {
            Ok(contents) => match validate_json(&contents) {
                Ok(_) => {
                    println!("JSON is valid!");
                    println!("Contents of the file:\n{}", contents);
                }
                Err(JsonParseError::InvalidStructure) => {
                    println!("Failed to parse JSON: Invalid structure.");
                    process::exit(1);
                }
                Err(JsonParseError::ParsingError(err)) => {
                    println!("Parsing error: {}", err);
                    process::exit(1);
                }
            },
            Err(e) => {
                println!("Could not read file {}: {}", file_path, e);
                process::exit(1);
            }
        },
    }
}
