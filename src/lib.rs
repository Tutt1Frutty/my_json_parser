use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// Custom error type for JSON parsing errors.
#[derive(Debug, Error)]
pub enum JsonParseError {
    #[error("Invalid JSON structure")]
    InvalidStructure,
    #[error("Parsing error: {0}")]
    ParsingError(String),
}

/// Define the Pest parser using the grammar in json.pest.
#[derive(Parser)]
#[grammar = "grammar/json.pest"]
struct JsonParser;

/// Validates if the input string is valid JSON according to the grammar.
pub fn validate_json(input: &str) -> Result<(), JsonParseError> {
    JsonParser::parse(Rule::json, input)
        .map(|_| ())
        .map_err(|e| JsonParseError::ParsingError(e.to_string()))
}

/// Parses JSON and identifies if it contains a specific object structure.
pub fn parse_object(input: &str) -> Result<(), JsonParseError> {
    let mut parsed = JsonParser::parse(Rule::object, input)
        .map_err(|e| JsonParseError::ParsingError(e.to_string()))?;

    if parsed.next().is_some() {
        println!("There is at least one more item in the iterator.");
    }

    Ok(())
}
