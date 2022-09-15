#[macro_use]
extern crate scan_fmt;

use std::process::exit;

use crate::errors::exit_with_retcode;

mod cli;
mod config;
mod container;
mod errors;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args))
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };
}
