use serde::{Deserialize, Serialize};
use serde_json::Result;
// use std::error::Error;
// use std::fs;
use std::fs::File;
// use std::io::BufReader;
use aho_corasick::{AhoCorasick, AhoCorasickBuilder};

// use csv::{ByteRecord, ReaderBuilder};

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

// fn parse_csv(path_csv: &str) -> Result<Stats> {
//     let f = BufReader::new(fs::File::open(&path_csv).unwrap());
//     let mut reader = ReaderBuilder::new().has_headers(false).from_reader(f);
//     let mut record = ByteRecord::new();
//     let total = 0;
//     let matching = 0;

//     while let Ok(true) = reader.read_byte_record(&mut record) {
//         println!("{:?}", record);
//     }

//     Ok(Stats { total, matching })
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

fn is_any_true(list_bool: &Vec<bool>) -> bool {
    list_bool.into_iter().any(|&m| m == true)
}

fn generator_aho_match(patterns: Vec<String>) -> AhoCorasick {
    AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .build(&*patterns)
}

fn is_match(ac: AhoCorasick, text: &str) -> bool {
    let matches: Vec<usize> = ac.find_iter(text).map(|mat| mat.pattern()).collect();
    !matches.is_empty()
}

fn is_match_multiple_condition(ac: AhoCorasick, total_pattern: usize, text: &str) -> bool {
    let matches: Vec<usize> = ac.find_iter(text).map(|mat| mat.pattern()).collect();
    matches.len() == total_pattern
}

fn run_match_multiple_condition(patterns: Vec<Vec<String>>, text: &str) -> bool {
    let matches: Vec<bool> = patterns
        .iter()
        .map(|p| {
            is_match_multiple_condition(generator_aho_match(p.to_vec()), p.to_vec().len(), text)
        })
        .collect();
    is_any_true(&matches)
}

#[allow(unused_variables)]
fn main() {
    let path_json = "./test.json";
    let message = "ไม่ส่งบ้านอ่อ🥺 เธอ hello test home hi good go house";
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

    // init aho
    let ac_patterns_include_condition = generator_aho_match(patterns_include_condition);
    let ac_patterns_exclude_condition = generator_aho_match(patterns_exclude_condition);

    assert_eq!(true, is_match(ac_patterns_exclude_condition, &message));
    assert_eq!(
        true,
        run_match_multiple_condition(patterns_exclude_multiple_condition, &message)
    );

    assert_eq!(true, is_match(ac_patterns_include_condition, &message));

    assert_eq!(
        true,
        run_match_multiple_condition(patterns_include_multiple_condition, &message)
    );
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
    fn test_is_any_true() {
        // Case true
        let dummy = vec![false, false, true];
        let result = is_any_true(&dummy);
        assert_eq!(true, result);

        // Case false:
        let dummy = vec![false, false, false];
        let result = is_any_true(&dummy);
        assert_eq!(false, result);
    }

    #[test]
    fn test_is_match() {
        let message = "hello test home";

        // Case found
        let dummy = vec!["test".to_string(), "home".to_string(), "word".to_string()];
        let mock_fn = generator_aho_match(dummy);
        let result = is_match(mock_fn, &message);
        assert_eq!(true, result);

        // Case not found
        let dummy = vec!["word".to_string()];
        let mock_fn = generator_aho_match(dummy);
        let result = is_match(mock_fn, &message);
        assert_eq!(false, result);
    }

    #[test]
    fn test_is_match_multiple_condition() {
        let message = "hello test home";

        // Case match all
        let dummy = vec!["test".to_string(), "home".to_string()];
        let mock_fn = generator_aho_match(dummy.clone());
        let result = is_match_multiple_condition(mock_fn, dummy.len(), &message);
        assert_eq!(true, result);

        // Case some match
        let dummy = vec!["store".to_string(), "home".to_string()];
        let mock_fn = generator_aho_match(dummy.clone());
        let result = is_match_multiple_condition(mock_fn, dummy.len(), &message);
        assert_eq!(false, result);
    }

    #[test]
    fn test_run_match_multiple_condition() {
        let message = "hello key word";
        // Case found
        let dummy = vec![
            vec!["book".to_string(), "monitor".to_string()],
            vec!["key".to_string(), "hello".to_string()],
        ];
        let result = run_match_multiple_condition(dummy, &message);
        assert_eq!(true, result);

        // Case not match all
        let dummy = vec![
            vec!["book2".to_string(), "monitor2".to_string()],
            vec!["key1".to_string(), "hello1".to_string()],
        ];
        let result = run_match_multiple_condition(dummy, &message);
        assert_eq!(false, result);
    }
}
