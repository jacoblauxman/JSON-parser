use crate::{JsonObject, JsonValue};
use std::collections::HashMap;

pub fn construct_thing() -> JsonObject {
    let mut obj = JsonObject {
        fields: HashMap::new(),
    };

    obj.fields.insert(
        "nested_obj".to_string(),
        JsonValue::Object(JsonObject {
            fields: HashMap::new(),
        }),
    );

    obj.fields
        .insert("boolean_val".to_string(), JsonValue::Bool(true));
    obj.fields
        .insert("int_val".to_string(), JsonValue::Number(420.69));
    obj.fields.insert("null_val".to_string(), JsonValue::Null);

    if let Some(JsonValue::Object(ref mut nested_obj)) = obj.fields.get_mut("nested_obj") {
        nested_obj
            .fields
            .insert("nested_arr".to_string(), JsonValue::Array(vec![]));

        nested_obj.fields.insert(
            "nested_nested_obj".to_string(),
            JsonValue::Object(JsonObject {
                fields: HashMap::new(),
            }),
        );
    };

    return obj;
}

pub fn construct_sam() -> JsonObject {
    let mut sam_json = JsonObject {
        fields: HashMap::new(),
    };

    sam_json
        .fields
        .insert("id".to_string(), JsonValue::Number(1.0));
    sam_json
        .fields
        .insert("name".to_string(), JsonValue::String("Sammich".to_string()));
    sam_json
        .fields
        .insert("age".to_string(), JsonValue::Number(5.0));
    sam_json
        .fields
        .insert("weight".to_string(), JsonValue::Number(13.7));
    sam_json.fields.insert(
        "favorites".to_string(),
        JsonValue::Array(vec![
            JsonValue::String("Crinkle Toys".to_string()),
            JsonValue::String("Greenies Treats".to_string()),
            JsonValue::String("Stealing Ben's food".to_string()),
        ]),
    );
    sam_json
        .fields
        .insert("microchipped".to_string(), JsonValue::Bool(false));
    sam_json
        .fields
        .insert("neutered".to_string(), JsonValue::Bool(true));
    sam_json
        .fields
        .insert("vaccinated".to_string(), JsonValue::Bool(true));

    let mut jaboc_json = JsonObject {
        fields: HashMap::new(),
    };

    jaboc_json
        .fields
        .insert("id".to_string(), JsonValue::Number(42.0));
    jaboc_json.fields.insert(
        "name".to_string(),
        JsonValue::String("Jaboc Laux".to_string()),
    );
    jaboc_json.fields.insert(
        "email".to_string(),
        JsonValue::String("jlauxman@gmail.com".to_string()),
    );
    jaboc_json.fields.insert(
        "phone".to_string(),
        JsonValue::Array(vec![
            JsonValue::Number(1234567.0),
            JsonValue::Number(9876543.0),
        ]),
    );

    sam_json
        .fields
        .insert("owner".to_string(), JsonValue::Object(jaboc_json));

    return sam_json;
}
