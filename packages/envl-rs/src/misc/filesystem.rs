use std::{fs::File, io::Read};

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
