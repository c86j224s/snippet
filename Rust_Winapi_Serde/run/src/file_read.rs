use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn read_file_all(file_name : &str) -> std::io::Result<String> {
    let mut file_data = String::new();

    let file = File::open(file_name)?;
    let mut buf_reader = std::io::BufReader::new(file);
    buf_reader.read_to_string(&mut file_data)?;
    Ok(file_data)
}
