use color_eyre::eyre::{Context, Result};
use pest::Parser;
use std::collections::HashMap;
use std::{fmt, fs};

#[derive(Parser)]
#[grammar = "grammar/systemd.pest"]
struct SystemdParser;

#[derive(Debug, Clone)]
pub enum SystemdValue {
    /// Wraps a String vector that contains multiple values for a duplicate key.
    List(Vec<String>),
    /// Wraps a String value of a respective key in the systemd unit file.
    Str(String),
}

impl fmt::Display for SystemdValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Type alias for HashMap<String, HashMap<String, SystemdValue>>.
pub type SystemdUnit = HashMap<String, HashMap<String, SystemdValue>>;

fn pre_process_map(map: &mut HashMap<String, HashMap<String, SystemdValue>>) {
    for (_, value) in map.into_iter() {
        for (_, v) in value.into_iter() {
            if let SystemdValue::List(vs) = v {
                if vs.len() == 0 {
                    let v_ = SystemdValue::Str(String::new());
                    *v = v_.clone();
                } else if vs.len() == 1 {
                    let v_ = SystemdValue::Str((vs[0]).clone());
                    *v = v_.clone();
                }
            }
        }
    }
}

pub fn parse(name: &str) -> Result<SystemdUnit> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    let unparsed_file =
        fs::read_to_string(name).wrap_err_with(|| format!("cannot read file {}", name))?;

    let file = SystemdParser::parse(Rule::file, &unparsed_file)
        .wrap_err("fail to parse")?
        .next()
        .unwrap();

    let mut properties: HashMap<String, HashMap<String, SystemdValue>> = HashMap::new();

    let mut current_section_name = String::new();
    let mut current_key_name = String::new();

    for line in file.into_inner() {
        match line.as_rule() {
            Rule::section => {
                let mut inner_rules = line.into_inner();
                current_section_name = inner_rules.next().unwrap().as_str().to_string();
            }
            Rule::property => {
                let mut inner_rules = line.into_inner();
                let section = properties.entry(current_section_name.clone()).or_default();

                let name = inner_rules.next().unwrap().as_str().to_string();
                let value = inner_rules.next().unwrap().as_str().to_string();

                if name == current_key_name {
                    let entry = section
                        .entry(current_key_name.clone())
                        .or_insert(SystemdValue::List(vec![]));
                    if let SystemdValue::List(ent) = entry {
                        ent.push(value);
                    }
                } else {
                    let entry = section
                        .entry(name.clone())
                        .or_insert(SystemdValue::List(vec![]));
                    if let SystemdValue::List(ent) = entry {
                        ent.push(value);
                    }
                    current_key_name = name;
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    pre_process_map(&mut properties);

    Ok(properties)
}
