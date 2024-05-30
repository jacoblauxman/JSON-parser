use json_parser::{construct_sam, construct_thing, parse_json};

fn main() {
    let good_input = include_str!("../tests/step4/valid2.json");
    let parsed = parse_json(good_input);

    println!(
        "\r\n--- --- parsed??? --- ---\r\n\n{:?}\r\n",
        parsed.clone().unwrap()
    );
    println!(
        "--- --- to string'D??? --- ---\r\n\n{}\r\n",
        parsed.unwrap().to_string()
    );

    // Test shape #1:
    let _obj = construct_thing();
    // println!("\r\nConstructed JSON OBJ: {}", _obj);

    //
    //

    // Test shape #2 (SAM):
    let _sam = construct_sam();
    println!("\r\n\nHere is Sammy (`Display` impl): \r\n\n {}\r\n", _sam);
}
