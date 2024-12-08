#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use advent2024::*;

    type Pos = (isize, isize);

    struct Pair(Pos, Pos);

    impl Pair {
        pub fn antinode(&self) -> [Pos; 2] {
            let Pair((ay, ax), (by, bx)) = self;
            let (dy, dx) = (by - ay, bx - ax);
            [(ay - dy, ax - dx), (by + dy, bx + dx)]
        }

        // I can't believe Rust do not have gcd in std....................
        fn gcd(mut n: isize, mut m: isize) -> isize {
            assert!(n != 0 && m != 0);
            while m != 0 {
                if m < n {
                    std::mem::swap(&mut m, &mut n);
                }
                m %= n;
            }
            n
        }

        pub fn resonant_antinode(&self) -> [impl Iterator<Item = Pos>; 2] {
            struct Antinode {
                start: Pos,
                d: Pos,
                multiplier: isize,
                step: isize,
            }

            impl Iterator for Antinode {
                type Item = Pos;

                fn next(&mut self) -> Option<Self::Item> {
                    let (ay, ax) = self.start;
                    let (dy, dx) = self.d;
                    let pos = (ay + self.multiplier * dy, ax + self.multiplier * dx);
                    self.multiplier += self.step;
                    Some(pos)
                }
            }

            let Pair((ay, ax), (by, bx)) = *self;
            let (mut dy, mut dx) = (by - ay, bx - ax);

            if dy == 0 {
                dx = 1
            } else if dx == 0 {
                dy = 1
            } else {
                let n = Self::gcd(dx.abs(), dy.abs());
                dy /= n;
                dx /= n;
            }

            [
                Antinode {
                    start: (ay, ax),
                    d: (dy, dx),
                    multiplier: 0,
                    step: 1,
                },
                Antinode {
                    start: (ay, ax),
                    d: (dy, dx),
                    multiplier: 0,
                    step: -1,
                },
            ]
        }
    }

    #[derive(Debug)]
    struct Map {
        antennas: HashMap<u8, Vec<Pos>>,
    }

    impl FromIterator<(u8, Pos)> for Map {
        fn from_iter<T: IntoIterator<Item = (u8, Pos)>>(iter: T) -> Self {
            let iter = iter.into_iter();
            let mut map = HashMap::with_capacity(36);
            iter.for_each(|(ty, pos)| {
                map.entry(ty)
                    .or_insert_with(|| Vec::with_capacity(128))
                    .push(pos);
            });
            Self { antennas: map }
        }
    }

    impl Map {
        pub fn pairs(&self) -> impl Iterator<Item = Pair> + '_ {
            self.antennas
                .values()
                .flat_map(|locations| {
                    locations.iter().enumerate().flat_map(|(idx, pos1)| {
                        locations.iter().skip(idx + 1).map(move |pos2| (pos1, pos2))
                    })
                })
                .map(|(a, b)| Pair(*a, *b))
        }
    }

    fn data() -> (Map, Pos /* Size of Map */) {
        let mut size = (0, 0);
        let map = read_by_line("tests/data/day8.input.txt")
            .enumerate()
            .inspect(|(row, line)| {
                size = (
                    size.0.max(*row as isize + 1),
                    size.1.max(line.trim_end().len() as isize),
                )
            })
            .flat_map(|(row, line)| {
                line.into_bytes()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, ty)| !matches!(ty, b'\r' | b'\n' | b'.'))
                    .map(move |(col, ty)| (ty, (row as isize, col as isize)))
            })
            .collect();
        (map, size)
    }

    fn check((py, px): &Pos, (sy, sx): &Pos) -> bool {
        *px >= 0 && *py >= 0 && px < sx && py < sy
    }

    #[test]
    fn part1() {
        let (map, size) = data();
        let locations: HashSet<Pos> = map
            .pairs()
            .flat_map(|pair| pair.antinode())
            .filter(|pos| check(pos, &size))
            .collect();

        assert_eq!(locations.len(), 367);
    }

    #[test]
    fn part2() {
        let (map, size) = data();
        let locations: HashSet<Pos> = map
            .pairs()
            .flat_map(|x| {
                let [pl, pr] = x.resonant_antinode();
                pl.take_while(|pos| check(pos, &size))
                    .chain(pr.take_while(|pos| check(pos, &size)))
            })
            .collect();

        assert_eq!(locations.len(), 1285);
    }
}
