use json_parser::{parse_json, JsonError};

fn main() -> Result<(), JsonError> {
    let good_input = include_str!("../tests/step5/valid.json");

    let parsed = parse_json(good_input);
    println!(
        "\n-- Tokenized input from test_input:\n{:?}",
        parsed.clone().unwrap()
    );
    println!(
        "-- Parsed input from test_input testing:\n{}\n",
        parsed.unwrap().to_string()
    );

    // SHOULD ERROR - testing bad inputs:
    let bad_input = include_str!("../tests/step5/invalid5.json");
    match parse_json(bad_input) {
        Ok(_) => println!("Shouldn't get this with the bad input :) :("),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }

    Ok(())
}
