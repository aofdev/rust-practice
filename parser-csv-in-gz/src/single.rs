extern crate csv;
extern crate flate2;

use std::env;
use std::fs::File;
use std::error::Error;
use std::str;

use flate2::read::GzDecoder;
use regex::Regex;
use csv::{ByteRecord, ReaderBuilder};

struct Stats {
    total: usize,
    matching: usize,
}

fn main() {
    let filter = Regex::new(r"(205).*").unwrap();
    let filenames = env::args().skip(1);
    let mut stats = Stats {
        total: 0,
        matching: 0,
    };

    for filename in filenames { 
        let result: Stats = count(&filename, &filter).expect(&format!("Couldn't read {}", &filename));
        stats.total += result.total;
        stats.matching += result.matching;
    }

    let percent = 100.0 * stats.matching as f32 / stats.total as f32;
    println!("match: {} / {} Total = {:.2}%", stats.matching, stats.total, percent);
}

fn count(path: &str, filter: &Regex) -> Result<Stats, Box<dyn Error>> {
    let mut input_file = File::open(path)?;
    let decoder = GzDecoder::new(&mut input_file);
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(decoder);
    
    let mut record = ByteRecord::new();
    let mut total = 0;
    let mut matching = 0;
 
    while let Ok(true) = reader.read_byte_record(&mut record) {
        total += 1;
        if let Some(bytes) = record.get(3) {
            let s = str::from_utf8(bytes)?;
            if filter.is_match(&s) {
                matching += 1;
            } 
        }
    }

    Ok(Stats { total, matching })

}