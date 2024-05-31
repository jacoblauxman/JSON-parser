use std::collections::HashMap;
use std::fmt::{self, Formatter};

#[derive(Debug, Clone)]
pub enum JsonError {
    InvalidSyntax,
    UnexpectedToken,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::InvalidSyntax => write!(f, "Invalid JSON Syntax"),
            JsonError::UnexpectedToken => write!(f, "Unexpected token in JSON input"),
        }
    }
}

impl std::error::Error for JsonError {}

#[derive(Debug, Clone)]
pub struct JsonObject {
    pub fields: HashMap<String, JsonValue>,
}

impl fmt::Display for JsonObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format_json(&JsonValue::Object(self.clone()), f, 1, true)
    }
}

#[derive(Debug, Clone)]
pub enum JsonValue {
    Object(JsonObject),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format_json(&self, f, 0, true)
    }
}

fn format_json(
    value: &JsonValue,
    f: &mut fmt::Formatter<'_>,
    indent_lvl: usize,
    top_level: bool,
) -> fmt::Result {
    let indent = "  ".repeat(indent_lvl);

    match value {
        JsonValue::Object(obj) => {
            if !top_level {
                write!(f, "\n{}", indent)?;
            }
            write!(f, "{{")?;

            let mut fields_it = obj.fields.iter().peekable();
            while let Some((key, val)) = fields_it.next() {
                write!(f, "\n  {}{}: ", indent, key)?;
                format_json(val, f, indent_lvl + 1, false)?;

                if fields_it.peek().is_some() {
                    write!(f, ",")?;
                }
            }

            if !top_level {
                write!(f, "\n{}}}", indent)
            } else {
                write!(f, "\n}}")
            }
        }
        JsonValue::Array(arr) => {
            write!(f, "[")?;

            let mut vals_it = arr.iter().peekable();
            while let Some(val) = vals_it.next() {
                format_json(val, f, indent_lvl + 1, false)?;

                if vals_it.peek().is_some() {
                    write!(f, ",")?;
                }
            }

            if top_level {
                write!(f, "\n]")
            } else if arr.iter().any(|val| matches!(val, JsonValue::Object(_))) {
                write!(f, "\n{}]", indent)
            } else {
                write!(f, "]")
            }
        }
        JsonValue::Bool(b) => write!(f, "{}", b),
        JsonValue::Number(n) => write!(f, "{}", n),
        JsonValue::String(s) => write!(f, "\"{}\"", s),
        JsonValue::Null => write!(f, "null"),
    }
}
