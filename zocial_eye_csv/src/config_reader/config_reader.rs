use std::{fs::File, io::BufReader};

use serde_json::from_reader;

use super::keyword_config::{ConfigStruct, KeywordConfig};


pub fn read_zocial_eye_config(json_path:&str)->Result<Vec<KeywordConfig>,Box<dyn std::error::Error>>{
    let file = File::open(json_path)?;
    let reader = BufReader::new(file);
    println!("reading config");
    let all_config:ConfigStruct = from_reader(reader)?;
    println!("config got!");
    Ok(all_config.get_keywords())



}