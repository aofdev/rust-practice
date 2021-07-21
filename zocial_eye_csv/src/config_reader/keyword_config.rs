use crate::matcher;
use serde::Deserialize;
enum InclusionSet {
    SingleKeyword(Vec<String>),
    MultipleKeyword(Vec<String>),
}
impl InclusionSet {
    pub fn new_group(keywords: &[String]) -> Vec<Self> {
        // split keywords into a + and non +
        let single_match = keywords
            .iter()
            .filter(|word| !word.contains('+'))
            .map(|word| word.to_string())
            // .map(|single_word| Self::SingleKeyword(single_word.clone()) )
            .collect::<Vec<String>>();
        let mut multiple_match = keywords
            .iter()
            .filter(|word| word.contains('+'))
            .map(|word| {
                word.split('+')
                    .map(|tmp_str| String::from(tmp_str))
                    .collect::<Vec<String>>()
            })
            .map(|each| Self::MultipleKeyword(each))
            .collect::<Vec<Self>>();

        multiple_match.push(Self::SingleKeyword(single_match));
        multiple_match
    }
}
enum ExclusionSet {
    SingleKeyword(Vec<String>),
    MultipleKeyword(Vec<String>), // Single(String),
                                  // Multiple(Vec<String>)
}
impl ExclusionSet {
    pub fn new_group(keywords: &[String]) -> Vec<Self> {
        // split keywords into a + and non +

        let single_match = keywords
            .iter()
            .filter(|word| !word.contains('+'))
            .map(|word| word.to_string())
            // .map(|single_word| Self::SingleKeyword(single_word.clone()) )
            .collect::<Vec<String>>();
        let mut multiple_match = keywords
            .iter()
            .filter(|word| word.contains('+'))
            .map(|word| {
                word.split('+')
                    .map(|tmp_str| String::from(tmp_str))
                    .collect::<Vec<String>>()
            })
            .map(|each| Self::MultipleKeyword(each))
            .collect::<Vec<Self>>();

        multiple_match.push(Self::SingleKeyword(single_match));
        multiple_match
    }
}

#[derive(Debug, Deserialize)]
pub struct KeywordConfig {
    name: String,
    include: Vec<String>,
    exclude: Vec<String>,
}
pub struct KeywordMatch {
    name: String,
    include: Vec<InclusionSet>,
    exclude: Vec<ExclusionSet>,
}

impl KeywordConfig {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn get_exclusion_set(&self) -> &[String] {
        &self.exclude
    }
    pub fn get_inclusion_set(&self) -> &[String] {
        &self.include
    }
}
#[derive(Debug, Deserialize)]
pub struct ConfigStruct {
    keywords: Vec<KeywordConfig>,
}
impl ConfigStruct {
    pub fn get_keywords(self) -> Vec<KeywordConfig> {
        self.keywords
    }
}
impl KeywordMatch {
    pub fn new(config: KeywordConfig) -> Self {
        let name = config.name;
        let include = InclusionSet::new_group(&config.include);
        let exclude = ExclusionSet::new_group(&config.exclude);
        Self {
            name,
            include,
            exclude,
        }
    }
    fn match_include(&self, message: &str) -> bool {
        for inclusion in &self.include {
            let is_match = match inclusion {
                InclusionSet::SingleKeyword(word) => matcher::is_match_find(&word, message),
                InclusionSet::MultipleKeyword(multiple_condition) => {
                    matcher::is_match_all_contains(
                        &multiple_condition,
                        &multiple_condition.len(),
                        message,
                    )
                }
            };
            if is_match {
                return true;
            }
        }
        false
    }
    fn match_exclude(&self, message: &str) -> bool {
        for exclusion in &self.exclude {
            let is_match = match exclusion {
                ExclusionSet::SingleKeyword(word) => matcher::is_match_find(&word, message),
                ExclusionSet::MultipleKeyword(multiple_condition) => {
                    matcher::is_match_all_contains(
                        &multiple_condition,
                        &multiple_condition.len(),
                        message,
                    )
                }
            };
            if is_match {
                return true;
            }
        }
        false
    }
    pub fn is_match(&self, message: &str) -> bool {
        self.match_include(message) && !self.match_exclude(message)
    }
    pub fn get_name(&self)->String {
        self.name.clone()
    }
}
#[test]
fn test_inclusion_set() {
    let config = KeywordConfig {
        name: "test".to_owned(),
        include: vec![
            "หมาน่อย".to_owned(),
            "หมาแม่ว".to_owned(),
            "แมว+น้อง".to_owned(),
        ],
        exclude: vec!["ม้า".to_owned()],
    };

    let keyword_match = KeywordMatch::new(config);
    assert_eq!(keyword_match.is_match(&"หมาแม่ว"), true);
    assert_eq!(keyword_match.is_match(&"น้อง"), false);
    assert_eq!(keyword_match.is_match(&"น้องหมาแมว แม่ววว"), true);
    assert_eq!(keyword_match.is_match(&"น้องหมาแมว แม่ววว ม้า"), false);
}
