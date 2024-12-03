use std::io::{BufRead, Read};

fn open(filename: &str) -> impl BufRead {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .open(filename)
        .unwrap();

    std::io::BufReader::new(f)
}

pub fn read_by_line(filename: &str) -> impl Iterator<Item = String> {
    let mut buffer = open(filename);
    std::iter::from_fn(move || {
        let mut line = String::new();
        let count = buffer.read_line(&mut line).ok()?;
        if count == 0 {
            None
        } else {
            Some(line)
        }
    })
}

pub fn read_by_byte(filename: &str) -> impl Iterator<Item = u8> {
    let mut buffer = open(filename);
    std::iter::from_fn(move || {
        let mut buf = [0u8; 1];
        buffer.read_exact(&mut buf).ok()?;
        Some(buf[0])
    })
}
