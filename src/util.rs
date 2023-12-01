use std::{
    fs::File,
    io::{Read, Write},
};

pub fn read_input(path: &str) -> std::io::Result<String> {
    let mut buf = String::new();
    let mut reader = File::open(path)?;

    let _ = reader.read_to_string(&mut buf);

    Ok(buf)
}
