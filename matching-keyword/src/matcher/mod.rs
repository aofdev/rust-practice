use aho_corasick::{AhoCorasick, AhoCorasickBuilder};
use rayon::prelude::*;
use std::io;
use std::str;

pub fn is_any_true(list_bool: &Vec<bool>) -> bool {
    list_bool.into_iter().any(|&m| m == true)
}

pub fn generator_aho_match(patterns: Vec<String>) -> AhoCorasick {
    AhoCorasickBuilder::new()
        .ascii_case_insensitive(true)
        .build(&*patterns)
}

pub fn is_match(ac: AhoCorasick, text: &str) -> bool {
    let matches: Vec<usize> = ac.find_iter(text).map(|mat| mat.pattern()).collect();
    !matches.is_empty()
}

#[allow(dead_code)]
pub fn is_match_contains(patterns: Vec<String>, text: &str) -> bool {
    let matches: Vec<bool> = patterns.iter().map(|t| text.contains(t)).collect();
    is_any_true(&matches)
}

#[allow(dead_code)]
pub fn is_match_contains_with_rayon(patterns: Vec<String>, text: &str) -> bool {
    let matches: Vec<bool> = patterns.par_iter().map(|t| text.contains(t)).collect();
    is_any_true(&matches)
}

#[allow(dead_code)]
pub fn is_match_with_bytes<R: io::Read>(ac: AhoCorasick, bytes_io: R) -> bool {
    ac.stream_find_iter(bytes_io).count() != 0
}

pub fn is_match_multiple_condition(ac: AhoCorasick, total_pattern: usize, text: &str) -> bool {
    let matches: Vec<usize> = ac.find_iter(text).map(|mat| mat.pattern()).collect();
    matches.len() == total_pattern
}

pub fn run_match_multiple_condition(patterns: Vec<Vec<String>>, text: &str) -> bool {
    let matches: Vec<bool> = patterns
        .iter()
        .map(|p| {
            is_match_multiple_condition(generator_aho_match(p.to_vec()), p.to_vec().len(), text)
        })
        .collect();
    is_any_true(&matches)
}

#[allow(dead_code)]
pub fn run_match_multiple_condition_with_rayon(patterns: Vec<Vec<String>>, text: &str) -> bool {
    let matches: Vec<bool> = patterns
        .par_iter()
        .map(|p| {
            is_match_multiple_condition(generator_aho_match(p.to_vec()), p.to_vec().len(), text)
        })
        .collect();
    is_any_true(&matches)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let message = "hello test home สวัสดีวันนี้อากาศร้อน123456789+*-)(~`~)@{.,}??<>$$##&|!/✆⍟🎉😆🇹🇭🇺🇸🧪🪐👩‍🚀❤️🔒 #me";

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
    fn test_is_match_contains() {
        let message = "hello test home สวัสดีวันนี้อากาศร้อน123456789+*-)(~`~)@{.,}??<>$$##&|!/✆⍟🎉😆🇹🇭🇺🇸🧪🪐👩‍🚀❤️🔒 #me";

        // Case found
        let dummy = vec!["test".to_string(), "home".to_string(), "word".to_string()];
        let result = is_match_contains(dummy, &message);
        assert_eq!(true, result);

        // Case not found
        let dummy = vec!["word".to_string()];
        let result = is_match_contains(dummy, &message);
        assert_eq!(false, result);
    }

    #[test]
    fn test_is_match_with_bytes() {
        // Case found
        let buf = io::BufReader::with_capacity(1, "My 🎉😆 home".as_bytes());
        let dummy = vec!["test".to_string(), "home".to_string(), "word".to_string()];
        let mock_fn = generator_aho_match(dummy);
        let result = is_match_with_bytes(mock_fn, buf);
        assert_eq!(true, result);

        // Case not found
        let buf = io::BufReader::with_capacity(1, "How does it work?".as_bytes());
        let dummy = vec!["test".to_string(), "home".to_string(), "word".to_string()];
        let mock_fn = generator_aho_match(dummy);
        let result = is_match_with_bytes(mock_fn, buf);
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
