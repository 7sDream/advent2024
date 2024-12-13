use std::io::{BufRead, Read};

fn open(filename: &str) -> impl BufRead {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .open(filename)
        .unwrap();

    std::io::BufReader::new(f)
}

pub fn read_all(filename: &str) -> Vec<u8> {
    let mut buffer = open(filename);
    let mut buf = Vec::with_capacity(1024);
    buffer.read_to_end(&mut buf).unwrap();
    buf
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

pub fn neighbors((row, col): (usize, usize)) -> [Option<(usize, usize)>; 4] {
    [
        if col > 0 { Some((row, col - 1)) } else { None },
        Some((row, col + 1)),
        if row > 0 { Some((row - 1, col)) } else { None },
        Some((row + 1, col)),
    ]
}

pub fn neighbors_limited(
    pos: (usize, usize),
    limit: (usize, usize),
) -> [Option<(usize, usize)>; 4] {
    let mut result = neighbors(pos);
    result.iter_mut().filter(|x| x.is_some()).for_each(|pos| {
        pos.take_if(|(row, col)| *row >= limit.0 || *col >= limit.1);
    });
    result
}
