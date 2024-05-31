use json_parser::{construct_sam, construct_thing, parse_json};

fn main() {
    let test_input = include_str!("../tests/step5/valid.json");

    let parsed = parse_json(test_input);
    println!(
        "\n-- Tokenized input from test_input:\n{:?}",
        parsed.clone().unwrap()
    );
    println!(
        "-- Parsed input from test_input testing:\n{}\n",
        parsed.unwrap().to_string()
    );

    // Test shape #1:
    let _obj = construct_thing();
    // println!("\n------------------------------------\n");
    // println!("\r\nConstructed JSON OBJ: {}", _obj);

    //
    //

    // Test shape #2 (SAM):
    let _sam = construct_sam();
    println!("\n------------------------------------\n");
    println!("\nHere is Sammy (`Display` impl): \n{}\n", _sam);
}
