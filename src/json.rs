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

// impl fmt::Display for JsonObject {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if self.fields.is_empty() {
//             write!(f, "{{}}")
//         } else {
//             write!(f, "{{\n")?;
//             let mut fields_it = self.fields.iter().peekable();
//             while let Some((key, val)) = fields_it.next() {
//                 write!(f, "\"{}\": {}", key, val)?;
//                 if fields_it.peek().is_some() {
//                     write!(f, ",\n")?;
//                 }
//             }
//             write!(f, " \n}}")
//         }
//     }
// }

impl fmt::Display for JsonObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        format_json(&JsonValue::Object(self.clone()), f, 0, true)
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

// impl fmt::Display for JsonValue {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             JsonValue::Object(obj) => write!(f, "{}", obj),
//             JsonValue::Array(arr) => {
//                 write!(f, "[")?;
//                 let mut vals_it = arr.iter().peekable();
//                 while let Some(val) = vals_it.next() {
//                     write!(f, "{}", val)?;
//                     if vals_it.peek().is_some() {
//                         write!(f, ", ")?;
//                     }
//                 }

//                 write!(f, "]")
//             }
//             JsonValue::Bool(b) => write!(f, "{}", b),
//             JsonValue::Number(n) => write!(f, "{}", n),
//             JsonValue::String(s) => write!(f, "\"{}\"", s),
//             JsonValue::Null => write!(f, "null"),
//         }
//     }
// }

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
                write!(f, "\n{}{}: ", indent, key)?;
                format_json(val, f, indent_lvl + 1, false)?;

                if fields_it.peek().is_some() {
                    write!(f, ",")?;
                }
            }

            if !top_level {
                write!(f, "\n{}", indent)?;
            }

            write!(f, "}}")
        }
        JsonValue::Array(arr) => {
            if !top_level {
                write!(f, "\n{}", indent)?;
            }
            write!(f, "[")?;

            let mut vals_it = arr.iter().peekable();
            while let Some(val) = vals_it.next() {
                // write!(f, "\r{}", indent)?; // todo: sort why this formats' desirable but deletes chars
                write!(f, "\n{}", indent)?;
                format_json(val, f, indent_lvl + 1, false)?;

                if vals_it.peek().is_some() {
                    write!(f, ",")?;
                }
            }

            if top_level {
                write!(f, "\n]")
            } else {
                write!(f, "\n{}]", indent)
            }
        }
        JsonValue::Bool(b) => write!(f, "{}", b),
        JsonValue::Number(n) => write!(f, "{}", n),
        JsonValue::String(s) => write!(f, "\"{}\"", s),
        JsonValue::Null => write!(f, "null"),
    }
}
