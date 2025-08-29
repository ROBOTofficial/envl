use std::{
    fs::File,
    io::{Error, Read},
};

pub fn read_file(file_path: String) -> Result<String, Error> {
    match File::open(file_path.to_owned()) {
        Ok(mut f) => {
            let mut buf = String::new();
            let _ = f.read_to_string(&mut buf);
            Ok(buf)
        }
        Err(err) => Err(err),
    }
}
