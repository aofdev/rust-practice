use csv::{ByteRecord, ReaderBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::str;

mod matcher;

#[derive(Debug, Serialize, Deserialize)]
struct Patterns {
    include: Vec<String>,
    name: String,
    exclude: Vec<String>,
}

fn parse_json(path_json: &str) -> Result<Patterns> {
    let file = File::open(&path_json).expect("file should open read only");
    let json: Patterns = serde_json::from_reader(file).expect("file should be proper JSON");
    Ok(json)
}

// fn parse_csv(path_csv: &str) {
//     let f = BufReader::new(fs::File::open(&path_csv).unwrap());
//     let mut reader = ReaderBuilder::new().has_headers(true).from_reader(f);
//     let mut record = ByteRecord::new();

//     while let Ok(true) = reader.read_byte_record(&mut record) {
//         if let Some(bytes) = record.get(0) {
//             let s = str::from_utf8(bytes);
//             if matcher()
//         }
//     }
// }

fn filter_condition(list_match: Vec<String>) -> (Vec<String>, Vec<Vec<String>>) {
    let mut list_match_temp = list_match;
    let mut multiple_condition: Vec<Vec<String>> = Vec::new();
    let multiple_condition_temp: Vec<String> = list_match_temp
        .iter()
        .filter(|&element| element.contains("+"))
        .cloned()
        .collect();

    for condition in multiple_condition_temp {
        multiple_condition.push(split_condition(&condition))
    }

    // Remove condition contains x
    list_match_temp.retain(|x| !x.contains("+"));

    (list_match_temp, multiple_condition)
}

fn split_condition(line: &str) -> Vec<String> {
    line.split("+").map(str::to_string).collect()
}

fn flatten_vector(nested_array: &Vec<Vec<String>>) -> Vec<String> {
    nested_array
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect()
}

fn concatenate_vector(patterns_one: &Vec<String>, patterns_two: &Vec<String>) -> Vec<String> {
    patterns_one
        .iter()
        .chain(patterns_two.iter())
        .cloned()
        .collect()
}

#[allow(unused_variables)]
fn main() {
    let path_json = "./test.json";
    let path_csv = "./message.test.csv";
    let res = parse_json(&path_json).expect("err parse_json");
    let patterns = Patterns {
        include: res.include,
        exclude: res.exclude,
        name: res.name,
    };

    let (patterns_include_condition, patterns_include_multiple_condition) =
        filter_condition(patterns.include);
    let (patterns_exclude_condition, patterns_exclude_multiple_condition) =
        filter_condition(patterns.exclude);

    // parse csv
    let f = BufReader::new(fs::File::open(&path_csv).unwrap());
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(f);
    let mut record = ByteRecord::new();

    while let Ok(true) = reader.read_byte_record(&mut record) {
        if let Some(bytes) = record.get(0) {
            let s = str::from_utf8(bytes).unwrap();
            // Check exclude
            if !matcher::execute(
                &patterns_exclude_condition,
                &patterns_exclude_multiple_condition,
                &s,
            ) {
                // Check include
                if matcher::execute(
                    &patterns_include_condition,
                    &patterns_include_multiple_condition,
                    &s,
                ) {
                    // TODO: set tags in the row
                    println!("found include word: {}", &s);
                }
            } else {
                println!("skip: found exclude word");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_condition() {
        let dummy = vec![
            "test".to_string(),
            "home".to_string(),
            "word+key".to_string(),
        ];
        let result = filter_condition(dummy);
        assert_eq!(
            (
                vec!["test".to_string(), "home".to_string()],
                vec![vec!["word".to_string(), "key".to_string()]]
            ),
            result
        );
    }
    #[test]
    fn test_flatten_vector() {
        let dummy = vec![
            vec!["test".to_string(), "home".to_string(), "key".to_string()],
            vec![
                "test2".to_string(),
                "home2".to_string(),
                "word2".to_string(),
            ],
        ];
        let actual = flatten_vector(&dummy);
        assert_eq!(
            vec![
                "test".to_string(),
                "home".to_string(),
                "key".to_string(),
                "test2".to_string(),
                "home2".to_string(),
                "word2".to_string(),
            ],
            actual
        );
    }

    #[test]
    fn test_concatenate_vector() {
        let dummy_one = vec!["test".to_string(), "home".to_string(), "word".to_string()];
        let dummy_two = vec!["test".to_string(), "angry".to_string(), "key".to_string()];
        let actual = concatenate_vector(&dummy_one, &dummy_two);
        assert_eq!(
            vec![
                "test".to_string(),
                "home".to_string(),
                "word".to_string(),
                "test".to_string(),
                "angry".to_string(),
                "key".to_string()
            ],
            actual
        );
    }
}
