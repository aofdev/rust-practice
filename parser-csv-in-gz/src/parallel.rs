extern crate csv;
extern crate flate2;
extern crate rayon;

use std::env;
use std::fs::File;
use std::error::Error;
use std::str;
use std::iter::Sum;

use flate2::read::GzDecoder;
use regex::Regex;
use csv::{ByteRecord, ReaderBuilder};
use rayon::prelude::*;
use regex::bytes::RegexBuilder;

struct Stats {
    total: usize,
    matching: usize,
}

impl Sum for Stats {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        let mut count = Stats { total: 0, matching: 0 };
        for stat in iter {
            count.total += stat.total;
            count.matching += stat.matching;
        }
        count
    }
}

fn main() {
    let pattern = r"(205).*";
    let regex = Regex::new(pattern).unwrap();

    let filenames: Vec<String> = env::args().skip(1).collect();

    let stats: Stats = filenames.par_iter()
        .map(|filename| match count(&filename, &regex) {
            Ok(stats) => stats,
            Err(error) => panic!("Problem: {:?}", error),
        })
        .sum();

    let percent = 100.0 * stats.matching as f32 / stats.total as f32;
    println!("match: {} / {} Total = {:.2}%", stats.matching, stats.total, percent);
}

#[allow(dead_code)]
fn count(path: &str, regex: &Regex) -> Result<Stats, Box<dyn Error>> {
    let mut input_file = File::open(&path)?;
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
            if regex.is_match(&s) {
                matching += 1;
            } 
        }
    }

    Ok(Stats { total, matching })

}

#[allow(dead_code)]
fn filter_regex(data: &str, pattern: &str) -> bool {
    let regex = RegexBuilder::new(pattern).unicode(false).build().unwrap();
    regex.is_match(data.as_bytes())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_regex_match() {
        let pattern = r"(205).*";
        const DATA: &str = "2052424222424205";
        let result = filter_regex(&DATA, &pattern);
        assert_eq!(true, result)
    }

    #[test]
    fn test_regex_not_match() {
        let pattern = r"(205).*";
        const DATA: &str = "3045832382438989";
        let result = filter_regex(&DATA, &pattern);
        assert_eq!(false, result)
    }
}