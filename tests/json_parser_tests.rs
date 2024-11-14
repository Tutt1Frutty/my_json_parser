use anyhow::Result;
use zhersh_json_parser::{parse_object, validate_json};

#[test]
fn test_valid_json_object() -> Result<()> {
    let input = r#"{ "name": "Maksym", "is_active": true }"#;
    validate_json(input)?;
    parse_object(input)?;
    Ok(())
}

#[test]
fn test_json_array() -> Result<()> {
    let input = r#"[1, 2, 3, 4, "text", false, null]"#;
    validate_json(input)?;
    Ok(())
}

#[test]
fn test_json_string() -> Result<()> {
    let input = r#""Hello, world!""#;
    validate_json(input)?;
    Ok(())
}

#[test]
fn test_json_number() -> Result<()> {
    let input = r#"123.456"#;
    validate_json(input)?;
    Ok(())
}

#[test]
fn test_invalid_json_structure() {
    let input = r#"{ "name": "Maksym", "is_active": true "#; // Missing closing brace
    assert!(validate_json(input).is_err());
}

#[test]
fn test_json_boolean_and_null() -> Result<()> {
    let input = r#"{"key": true, "another_key": null}"#;
    validate_json(input)?;
    Ok(())
}
