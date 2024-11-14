# Zhersh JSON Parser

## Overview
Zhersh JSON Parser is a simple command-line JSON parser written in Rust. It validates JSON files based on a custom grammar and outputs the file content if the structure is correct.

## Technical Description
The parser uses Pest for parsing JSON files. It follows basic JSON grammar rules for strings, numbers, arrays, objects, booleans, and null values. The parser checks the structure of JSON files for validity and provides error messages if the format is incorrect.

## Usage
```bash
cargo run -- zhersh_json.json       # Parses the specified JSON file
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
cargo run -- zhersh_json.json
```

Example of output:
```
JSON is valid!
Contents of the file:
{
  "name": "zhersh_json_parser",
  "version": "1.0",
  "is_active": true,
...
```

Displaying help:
```bash
cargo run -- --help
```

Output:
```
Usage:
  json_parser zhersh_json.json   Parses the specified JSON file.
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