# Zhersh JSON Parser

## Overview
Zhersh JSON Parser is a simple command-line JSON parser written in Rust. It validates JSON files based on a custom grammar and outputs the file content if the structure is correct.

docs.rs link: https://docs.rs/my_json_parser_proj/0.1.1/my_json_parser_proj/
crates.io link: https://crates.io/crates/my_json_parser_proj

## Technical Description
The parser uses Pest for parsing JSON files. It follows basic JSON grammar rules for strings, numbers, arrays, objects, booleans, and null values. The parser checks the structure of JSON files for validity and provides error messages if the format is incorrect.

## Usage
```bash
cargo run -- my_json.json       # Parses the specified JSON file
cargo run -- --help                 # Displays help information
cargo run -- --credits              # Shows project credits
```

## Building the Project
Run the following command to build the project:
```bash
cargo build
```

## Running the project
To run the project use this command:
```bash
cargo run -- <file.json>
```
For example:
```bash
cargo run -- my_json.json
```

Example of output:
```
JSON is valid!
Contents of the file:
{
  "name": "my_json_parser_proj",
  "version": "1.1",
  "is_active": true,
  ...
    "updated_at": "2024-11-19T14:32:00Z",
...
```

Displaying help:
```bash
cargo run -- --help
```

Output:
```
Usage:
  json_parser my_json.json   Parses the specified JSON file.
  json_parser --help        Displays this help information.
  json_parser --credits     Shows credits for this project.
```

Displaying credits:
```bash
cargo run -- --credits
```

Output:
```
JSON Parser Project
Author: Maksym Zhersh (AM-3)
Repository: https://github.com/Tutt1Frutty/my_json_parser.git
```

## JSON grammar
The JSON parser follows a simplified grammar:
```
object: { "key": value, ... }
array: [value, ...]
string: "text"
number: 123, -123, 3.14
boolean: true or false
null: null
date: "2024-11-11T10:40:00Z" in ISO 8601 format
```

## Makefile commands
This project includes a Makefile. Common commands:
``` 
make run file=<file>: Run the parser with a specified JSON file.
make test: Run all unit tests.
make format: Format the code with cargo fmt.
make lint: Lint the code with cargo clippy.
make check: Run all checks before committing.
```

## Tests
Unit tests are included in the tests/json_parser_tests.rs file. To run the tests:
```bash
cargo test
```
