use std::error::Error;
use std::io;

use csv::{ByteRecord, ReaderBuilder};
use regex::bytes::RegexBuilder;

struct Stats {
    total: usize,
    matching: usize,
}

fn main() {
    let result: Stats = count().unwrap_or_else(|_| panic!("Couldn't read"));
    let percent = 100.0 * result.matching as f32 / result.total as f32;
    println!(
        "match: {} / {} Total = {:.2}%",
        result.matching, result.total, percent
    );
}

fn count() -> Result<Stats, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    let regex = RegexBuilder::new(r"(205).*")
        .unicode(false)
        .build()
        .unwrap();
    let mut record = ByteRecord::new();
    let mut total = 0;
    let mut matching = 0;

    while let Ok(true) = reader.read_byte_record(&mut record) {
        total += 1;
        if regex.is_match(&record[3]) {
            matching += 1;
        }
    }

    Ok(Stats { total, matching })
}
