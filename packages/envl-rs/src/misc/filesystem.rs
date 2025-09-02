use std::{
    fs::File,
    io::{Error, Read, Write},
    path::Path,
};

use crate::misc::error::{convert_io_error, EnvlError};

pub fn read_file(file_path: String) -> Result<String, EnvlError> {
    match File::open(file_path.to_owned()) {
        Ok(mut f) => {
            let mut buf = String::new();
            let _ = f.read_to_string(&mut buf);
            Ok(buf)
        }
        Err(err) => Err(convert_io_error(err)),
    }
}

pub fn write_file(file_path: String, txt: String) -> Result<usize, Error> {
    let f = if Path::new(&file_path).is_file() {
        File::open(file_path)
    } else {
        File::create(file_path)
    };
    match f {
        Ok(mut f) => f.write(&txt.as_bytes()),
        Err(err) => Err(err),
    }
}
