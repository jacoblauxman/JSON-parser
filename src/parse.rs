use crate::{JsonError, JsonObject, JsonValue};
use std::collections::HashMap;

pub fn parse_json(input: &str) -> Result<JsonValue, JsonError> {
    println!("input provided to parse_json: {}", input);
    let input = input.trim();

    if input.starts_with('{') && input.ends_with('}') {
        parse_object(input)
    } else if input.starts_with('[') && input.ends_with(']') {
        parse_array(input)
    } else {
        parse_value(input)
    }
}

pub fn parse_value(input: &str) -> Result<JsonValue, JsonError> {
    println!("input provided to parse_value: {}", input);

    if input.starts_with('"') && input.ends_with('"') {
        Ok(JsonValue::String(input[1..input.len() - 1].to_string())) // remove actual apostrophes (" ")
    } else if let Ok(n) = input.parse::<f64>() {
        Ok(JsonValue::Number(n))
    } else if input == "true" {
        Ok(JsonValue::Bool(true))
    } else if input == "false" {
        Ok(JsonValue::Bool(false))
    } else if input.starts_with('{') && input.ends_with('}') {
        parse_object(input)
    } else if input.starts_with('[') && input.ends_with(']') {
        parse_array(input)
    } else if input == "null" {
        Ok(JsonValue::Null)
    } else {
        Err(JsonError::UnexpectedToken)
    }
}

pub fn parse_object(input: &str) -> Result<JsonValue, JsonError> {
    println!("input provided to parse_object: {}", input);

    let input = input.trim();
    if input == "{}" {
        return Ok(JsonValue::Object(JsonObject {
            fields: HashMap::new(),
        }));
    }

    let mut fields = HashMap::new();
    let inner = &input[1..input.len() - 1].trim();
    let inner_parts = inner.split(',').collect::<Vec<&str>>();

    println!("INNER PARTS parse_obj: {:?}", inner_parts);

    for part in inner_parts {
        // println!("part in parse_obj: {:?}", part);
        let kv = part.trim().split(':').collect::<Vec<&str>>();
        println!("kv in parse_obj: {:?}", kv);
        // if kv.len() != 2 {
        if kv.len() < 2 {
            return Err(JsonError::InvalidSyntax);
        }

        let key = kv[0].trim().trim_matches('"').to_string();
        // let val = parse_value(kv[1].trim())?;
        let val = parse_value(kv[1..].join(":").trim())?;

        fields.insert(key, val);
    }

    Ok(JsonValue::Object(JsonObject { fields }))
}

pub fn parse_array(input: &str) -> Result<JsonValue, JsonError> {
    println!("input provided to parse_array: {}", input);

    let input = input.trim();

    if input == "[]" {
        return Ok(JsonValue::Array(vec![]));
    }

    let mut vals = vec![];
    let inner = &input[1..input.len() - 1];
    let inner_parts = inner.split(',').collect::<Vec<&str>>();

    for part in inner_parts {
        vals.push(parse_value(part.trim())?);
    }

    Ok(JsonValue::Array(vals))
}
