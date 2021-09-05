extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;
mod parser_json;
use color_eyre::eyre::Result;
use std::{env::current_dir, fs};

use parser::parse;
use parser_json::{parse_json_file, serialize_jsonvalue};

fn main() -> Result<()> {
    color_eyre::install()?;

    // Parse systemd file
    let dir = current_dir().unwrap().join("files/cardano-node.service");
    let filepath = dir.to_str().unwrap();
    if let Ok(p) = parse(filepath) {
        let unit = p.get(&String::from("Unit")).unwrap();
        println!(
            "Description: {}",
            unit.get(&"Description".to_string()).unwrap()
        );
        println!("Wants: {}", unit.get(&"Wants".to_string()).unwrap());
    }

    // Parse JSON file
    let dir = current_dir().unwrap().join("files/test.json");
    let filepath = dir.to_str().unwrap();
    let unparsed_file = fs::read_to_string(filepath).expect("cannot read file");
    let json = parse_json_file(&unparsed_file).expect("unsuccessful parse");

    let result = serialize_jsonvalue(&json);
    println!("{:?}", result);
    Ok(())
}
