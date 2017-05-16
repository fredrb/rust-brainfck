use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub enum LoadError {
    FailedToOpen,
    FailedToRead,
    InvalidProgram,
}

pub type LoadResult = Result<String,LoadError>;

pub fn load_program_from_path(path: &'static str) -> LoadResult {
    let path = Path::new(path);

    let mut file = match File::open(&path) {
        Err(_) => return Err(LoadError::FailedToOpen),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(_) => return Err(LoadError::FailedToRead),
        Ok(_) => return Ok(s),
    }
}
