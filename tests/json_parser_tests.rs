use anyhow::Result;
use my_json_parser_proj::{parse_object, validate_json};

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

#[test]
fn test_exponential_number_positive() -> Result<()> {
    let input = r#"1.23e+10"#;
    validate_json(input)?;
    Ok(())
}

#[test]
fn test_exponential_number_negative() -> Result<()> {
    let input = r#"-5e-2"#;
    validate_json(input)?;
    Ok(())
}

#[test]
fn test_invalid_exponential_number() {
    let input = r#"123e"#; // Missing exponent value
    assert!(validate_json(input).is_err());
}

#[test]
fn test_valid_iso_date() -> Result<()> {
    let input = r#""2023-11-20T14:32:00Z""#;
    validate_json(input)?;
    Ok(())
}
