use crate::{JsonError, JsonObject, JsonValue};
use std::collections::HashMap;

pub fn parse_json(input: &str) -> Result<JsonValue, JsonError> {
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
    let input = input.trim();
    if input == "{}" {
        return Ok(JsonValue::Object(JsonObject {
            fields: HashMap::new(),
        }));
    }

    let mut fields = HashMap::new();
    let inner = &input[1..input.len() - 1].trim();
    // let inner_parts = inner.split(',').collect::<Vec<&str>>();
    let inner_parts = split_input(inner, ',');

    for part in inner_parts {
        // let kv = part.trim().split(':').collect::<Vec<&str>>();
        let kv = split_input(part, ':');
        // if kv.len() < 2 {
        if kv.len() != 2 {
            return Err(JsonError::InvalidSyntax);
        }

        let key = kv[0].trim().trim_matches('"').to_string();
        // let val = parse_value(kv[1].trim())?;
        let val = parse_value(kv[1].trim())?;

        fields.insert(key, val);
    }

    Ok(JsonValue::Object(JsonObject { fields }))
}

pub fn parse_array(input: &str) -> Result<JsonValue, JsonError> {
    // println!("input provided to parse_array: {}", input);

    let input = input.trim();

    if input == "[]" {
        return Ok(JsonValue::Array(vec![]));
    }

    let mut vals = vec![];
    let inner = &input[1..input.len() - 1];
    // let inner_parts = inner.split(',').collect::<Vec<&str>>();
    let inner_parts = split_input(inner, ',');

    for part in inner_parts {
        vals.push(parse_value(part.trim())?);
    }

    Ok(JsonValue::Array(vals))
}

fn split_input(input: &str, delim: char) -> Vec<&str> {
    let mut items = vec![];
    let mut in_string = false;
    let mut brace_lvl = 0;
    let mut bracket_lvl = 0;
    let mut start = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '"' => in_string = !in_string,
            '{' if !in_string => brace_lvl += 1,
            '}' if !in_string => brace_lvl -= 1,
            '[' if !in_string => bracket_lvl += 1,
            ']' if !in_string => bracket_lvl -= 1,
            c if c == delim && !in_string && brace_lvl == 0 && bracket_lvl == 0 => {
                items.push(&input[start..i]);
                start = i + 1;
            }
            _ => (),
        }
    }

    items.push(&input[start..]);
    items
}
