use std::io::{Error, ErrorKind};

use envl_config::misc::config::Config;

use crate::generate_typefile::rust::gen_rust_typefile;

pub mod rust;

pub fn generate_typefile(config: Config, output: String) -> Result<String, Error> {
    if output.ends_with("rs") {
        gen_rust_typefile(config)
    } else {
        Err(Error::new(ErrorKind::Other, "Unsupported file"))
    }
}
