use envfile::EnvFile;
use std::path::Path;

pub trait IConfig {
    fn get_config_with_key(&self, key: &str) -> String;
}

pub struct Config {}

impl IConfig for Config {
    fn get_config_with_key(&self, key: &str) -> String {
        let env = EnvFile::new(&Path::new(".env")).expect("Failed to load .env file");
        env.get(key)
            .expect(format!("Failed to get key `{}` in .env file", key).as_str())
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_config_with_key() {
        let config = Config {};
        assert_eq!(config.get_config_with_key("APP_HOST"), "localhost");
    }

    #[test]
    #[should_panic(expected = "Failed to get key `DB` in .env file")]
    fn test_get_config_with_key_not_found_key() {
        let config = Config {};
        config.get_config_with_key("DB");
    }
}
