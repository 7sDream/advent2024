#[cfg(test)]
mod tests {
    use advent2024::*;

    #[derive(Debug)]
    struct Schematic([u8; 5]);

    impl Schematic {
        pub fn fits(&self, other: &Self) -> bool {
            self.0.iter().zip(other.0.iter()).all(|(a, b)| a + b <= 5)
        }
    }

    #[derive(Debug)]
    struct Schematics {
        locks: Vec<Schematic>,
        keys: Vec<Schematic>,
    }

    impl Schematics {
        pub fn fits_count(&self) -> usize {
            self.locks
                .iter()
                .flat_map(|lock| self.keys.iter().filter(|key| lock.fits(key)))
                .count()
        }
    }

    fn data(path: &str) -> Schematics {
        let mut lines = read_by_line(path).peekable();
        let mut locks = vec![];
        let mut keys = vec![];

        while lines.peek().is_some() {
            let block = lines.by_ref().take(7);
            let mut target = &mut locks;
            let mut schematic = [0; 5];
            for (i, line) in block.enumerate() {
                if i == 0 {
                    if line.trim() == "#####" {
                        target = &mut locks;
                    }
                } else if i == 6 {
                    if line.trim() == "#####" {
                        target = &mut keys;
                    }
                    target.push(Schematic(schematic));
                } else {
                    line.trim()
                        .as_bytes()
                        .iter()
                        .enumerate()
                        .filter(|(_, b)| **b == b'#')
                        .for_each(|(i, _)| {
                            schematic[i] += 1;
                        });
                }
            }
            let _ = lines.next(); // skip empty line
        }

        Schematics { locks, keys }
    }

    #[test]
    fn part1() {
        let schematics = data("tests/data/day25.input.txt");
        assert_eq!(schematics.fits_count(), 3201);
    }
}
