use std::io::BufRead;

pub fn read_by_line(filename: &str) -> impl Iterator<Item = String> {
    let f = std::fs::OpenOptions::new()
        .read(true)
        .open(filename)
        .unwrap();
    let mut buffer = std::io::BufReader::new(f);
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
