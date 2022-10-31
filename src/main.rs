use bson::{bson, Document};
use clap::Parser;
use serde_json::Value;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};

mod bson_json_processor;

use bson_json_processor::{bson_to_simple_json, json_into_bson};

#[derive(Debug, Parser)]
#[command(name = "bs")]
#[command(about = "bson decoder and encoder CLI", long_about = None)]
struct Cli {
    #[clap(short, long)]
    path: Option<String>,
    #[clap(short, long)]
    decode: Option<bool>,
    #[clap(short, long)]
    verbose: Option<bool>,
}

fn main() {
    let args = Cli::parse();
    let reader: Box<dyn BufRead> = match args.path {
        None => Box::new(BufReader::new(io::stdin())),
        Some(path) => Box::new(BufReader::new(fs::File::open(path).unwrap())),
    };
    // let verbose = if let Some(true) = args.verbose { true } else { false };
    if let Some(true) = args.decode {
        let doc = Document::from_reader(reader).expect("Failed to parse given data");
        let json_str = bson_to_simple_json(&bson!(doc)).expect("Failed to generate JSON from BSON");
        println!("{}", json_str);
    } else {
        let json: Value = serde_json::from_reader(reader).expect("Failed to parse given data");
        let bson_binary = json_into_bson(&json).expect("Failed to generate BSON from JSON");
        std::io::stdout().write_all(&bson_binary).expect("Failed to write");
    }
}
