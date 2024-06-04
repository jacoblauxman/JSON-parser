use clap::Parser;
use json_parser::{parse_json, JsonError};
use std::{fs::File, io::Read, path::Path};

const DEFAULT_PATH: &'static str = "output.json";

#[derive(Parser, Debug)]
struct Args {
    #[arg(
        short = 'i',
        long = "input",
        help = "The input file path for reading JSON data"
    )]
    input: Option<String>,
    #[arg(
        short = 'j',
        long = "json",
        help = "Use this for direct JSON string input"
    )]
    json_string: Option<String>,
    #[arg(
        short = 'o',
        long = "output",
        help = "Writes the parsed JSON data to `output.json` (in the project root directory)."
    )]
    output: bool,
    #[arg(
        short = 't',
        long = "tokenized",
        help = "Additionally prints the program's tokenized values from parsed JSON data"
    )]
    tokenized: bool,
    #[arg(
        short = 'q',
        long = "quiet",
        help = "Supresses printing results to terminal window."
    )]
    quiet: bool,
}

fn main() -> Result<(), JsonError> {
    let args = Args::parse();

    if args.input.is_none() && args.json_string.is_none() {
        println!("No input data provided. Provide either an input file path or a JSON string.");
        return Ok(());
    }

    let input_data = if let Some(json_string) = args.json_string {
        json_string
    } else {
        let input_path = match args.input {
            Some(ref input) => match Path::new(input).canonicalize() {
                Ok(path) => path,
                Err(err) => {
                    eprintln!("Failed to get absolute path to JSON file: {}", err);
                    return Err(JsonError::FileRead(args.input.unwrap()));
                }
            },
            None => {
                println!("No file path input provided. Specify either an input file path or provide a JSON string.");
                return Ok(());
            }
        };

        let mut input_file = match File::open(&input_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to open input JSON file: {}", err);
                return Err(JsonError::FileRead(args.input.unwrap()));
            }
        };

        let mut input_data = String::new();

        if let Err(err) = input_file.read_to_string(&mut input_data) {
            eprintln!("Failed to read data from JSON file path: {}", err);
            return Err(JsonError::FileRead(args.input.unwrap()));
        };

        input_data
    };

    let parsed_data = parse_json(&input_data)?;

    if !args.quiet {
        if args.tokenized {
            println!("Tokenized data from input:\n{:?}\n", parsed_data.clone());
        }
        println!("Parsed data from input:\n{}\n", &parsed_data.to_string());
    }

    if args.output {
        let path = Path::new(DEFAULT_PATH);
        if let Err(_) = std::fs::write(path, parsed_data.to_string()) {
            return Err(JsonError::FileWrite);
        }
    } else {
        let path = Path::new(&"output.json");
        if let Err(_) = std::fs::write(path, parsed_data.to_string()) {
            return Err(JsonError::FileWrite);
        }
    }

    Ok(())
}
