use std::io::{Error, ErrorKind};

use crate::{generator::rust::generate_rust_file, VariableHashMap};

pub mod rust;

pub fn generate_file(data: VariableHashMap, output: String) -> Result<String, Error> {
    if output.ends_with("rs") {
        generate_rust_file(data)
    } else {
        Err(Error::new(ErrorKind::Other, "Unsupported file"))
    }
}
