use std::io::{Error, ErrorKind};

use envl_config::misc::config::Config;

use crate::generator::rust::generate_rust_file;

pub mod rust;

pub fn generate_file(config: Config, output: String) -> Result<String, Error> {
    if output.ends_with("rs") {
        generate_rust_file(config)
    } else {
        Err(Error::new(ErrorKind::Other, "Unsupported file"))
    }
}
