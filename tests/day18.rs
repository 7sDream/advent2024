#[cfg(test)]
mod tests {
    use std::{
        cmp::Ordering,
        collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
        fmt::Write,
    };

    use advent2024::*;

    #[derive(Debug, Clone, Copy)]
    enum Tile {
        Empty,
        Corrupted,
    }

    #[derive(Debug)]
    struct Memory {
        size: (usize, usize),
        tiles: Vec<Vec<Tile>>,
    }

    impl std::fmt::Display for Memory {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for line in &self.tiles {
                for tile in line {
                    match tile {
                        Tile::Empty => f.write_char('.')?,
                        Tile::Corrupted => f.write_char('#')?,
                    }
                }
                f.write_char('\n')?;
            }
            Ok(())
        }
    }

    impl Memory {
        fn start(&self) -> (usize, usize) {
            (0, 0)
        }

        fn end(&self) -> (usize, usize) {
            let (y, x) = self.size;
            (
                y.checked_sub(1).unwrap_or_default(),
                x.checked_sub(1).unwrap_or_default(),
            )
        }

        fn byte_fall_at(&mut self, (y, x): (usize, usize)) {
            self.tiles[y][x] = Tile::Corrupted;
        }

        fn path(&self) -> Option<Vec<(usize, usize)>> {
            let mut q = VecDeque::new();
            let mut s = HashMap::<(usize, usize), (usize, HashSet<(usize, usize)>)>::new();
            q.push_back((0, self.start()));
            s.insert(self.start(), (0, HashSet::new()));

            while let Some((score, (y, x))) = q.pop_front() {
                if matches!(self.tiles[y][x], Tile::Corrupted) {
                    continue;
                }
                if s.get(&(y, x)).unwrap().0 != score {
                    continue;
                }
                let score = score + 1;
                for neighbor in neighbors_limited((y, x), self.size).into_iter().flatten() {
                    let entry = s.entry(neighbor);
                    match entry {
                        Entry::Vacant(e) => {
                            e.insert((score, Some((y, x)).into_iter().collect()));
                            q.push_back((score, neighbor));
                        }
                        Entry::Occupied(mut e) => {
                            let (last, parents) = e.get_mut();
                            match score.cmp(last) {
                                Ordering::Less => {
                                    e.insert((score, Some((y, x)).into_iter().collect()));
                                    q.push_back((score, neighbor));
                                }
                                Ordering::Equal => {
                                    parents.insert((y, x));
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }

            let mut path = vec![self.end()];
            let mut current = self.end();
            while current != self.start() {
                if let Some((_, parents)) = s.get(&current) {
                    current = parents.iter().copied().next().unwrap();
                    path.push(current);
                } else {
                    return None;
                }
            }

            path.reverse();

            Some(path)
        }
    }

    fn data(path: &str, size: (usize, usize)) -> (Memory, impl Iterator<Item = (usize, usize)>) {
        let tiles = vec![vec![Tile::Empty; size.1]; size.0];
        let it = read_by_line(path).map(|line| {
            let mut parts = line
                .trim()
                .split(',')
                .map(|part| part.parse::<usize>().unwrap());
            let (x, y) = (parts.next().unwrap(), parts.next().unwrap());
            (y, x)
        });

        (Memory { size, tiles }, it)
    }

    #[test]
    fn part1() {
        let (mut memory, falls) = data("tests/data/day18.input.txt", (71, 71));
        falls.take(1024).for_each(|pos| memory.byte_fall_at(pos));
        assert_eq!(memory.path().unwrap().len() - 1, 260);
    }

    // Not a very efficient way, but it works. And I'm tired today, so I won't optimize it anymore...
    #[test]
    fn part2() {
        let (mut memory, mut falls) = data("tests/data/day18.input.txt", (71, 71));
        let first_broken = falls.find(|pos| {
            memory.byte_fall_at(*pos);
            memory.path().is_none()
        });
        // This is (y, x), but in website we need input it as x,y
        assert_eq!(first_broken, Some((48, 24)));
    }
}
