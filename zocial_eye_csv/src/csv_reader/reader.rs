use csv::{Reader,ReaderBuilder};
use flate2::read::GzDecoder;
use std::{error::Error, fs::File, io::{BufReader, prelude::*}, path};
use rayon::prelude::*;

pub fn read_csv(file_path:&str) -> Result<Reader<File>,Box<dyn Error>> {
    Ok(ReaderBuilder::new().has_headers(true).quoting(true).double_quote(false).from_path(file_path)?)

    
}