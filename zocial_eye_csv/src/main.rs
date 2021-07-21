use csv::{ByteRecord, ByteRecordsIntoIter, ByteRecordsIter, Writer, WriterBuilder};
use regex::bytes::{Regex, RegexBuilder};
use std::{
    any::Any,
    collections::HashMap,
    error::Error,
    fs::File,
    iter::Take,
    path::{Path, PathBuf},
};

use zocial_eye_csv::csv_reader::reader::read_csv;
use zocial_eye_csv::{
    config_reader::{config_reader::read_zocial_eye_config, keyword_config::KeywordMatch},
    zocial_eye_data::zocial_data::{ZocialData, FIELD_NAMES},
};

fn main() -> Result<(), Box<dyn Error>> {
    let config_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("./config.json");

    let config = read_zocial_eye_config(config_path.to_str().unwrap())?;
    let keywords: Vec<KeywordMatch> = config
        .into_iter()
        .map(|config| KeywordMatch::new(config))
        .collect();
    let target_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("./ze_message.csv");
    let output_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("./output.csv");
    let field_names = ByteRecord::from(Vec::from(FIELD_NAMES.clone()));
    let mut file = read_csv(target_path.to_str().unwrap())?;
    let mut output_file = WriterBuilder::new()
        .has_headers(true)
        .from_path(output_file.to_str().unwrap())?;
    let lines_result = file.byte_records();
    for record_result in lines_result {
        if let Ok(record) = record_result {
            let mut row: ZocialData = record.deserialize(Some(&field_names))?;
            keywords.iter().for_each(|keyword_set| {
                row.process_match_keyword(keyword_set);
            });

            output_file.serialize(row)?;
        } else if let Err(err) = record_result {
            return Err(Box::from(err));
        }
    }
    Ok(())
}
