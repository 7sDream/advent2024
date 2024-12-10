#[cfg(test)]
mod tests {
    use std::collections::{hash_map::Entry, HashMap, VecDeque};

    use advent2024::*;

    #[derive(Debug)]
    struct Tile {
        height: u8,
        score: usize,
        rating: usize,
    }

    #[derive(Debug)]
    struct Map(Vec<Vec<Tile>>);

    impl Map {
        pub fn new(data: Vec<Vec<u8>>) -> Self {
            let map: Vec<_> = data
                .into_iter()
                .map(|x| {
                    x.into_iter()
                        .map(|height| Tile {
                            height,
                            score: 0,
                            rating: 0,
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            let tops: Vec<_> = map
                .iter()
                .enumerate()
                .flat_map(|(row, line)| {
                    line.iter()
                        .enumerate()
                        .filter(|(_, tile)| tile.height == 9)
                        .map(move |(col, _)| (row, col))
                })
                .collect();

            let mut this = Map(map);

            tops.into_iter().for_each(|pos| {
                this.update_score_rating(pos);
            });

            this
        }

        pub fn trailheads(&self) -> impl Iterator<Item = &Tile> {
            self.0
                .iter()
                .flat_map(|line| line.iter())
                .filter(|x| x.height == 0)
        }

        fn neighbors(&self, (row, col): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
            [
                if col > 0 { Some((row, col - 1)) } else { None },
                if col + 1 < self.0[row].len() {
                    Some((row, col + 1))
                } else {
                    None
                },
                if row > 0 { Some((row - 1, col)) } else { None },
                if row + 1 < self.0.len() {
                    Some((row + 1, col))
                } else {
                    None
                },
            ]
            .into_iter()
            .flatten()
        }

        fn update_score_rating(&mut self, pos: (usize, usize)) {
            let mut q = VecDeque::with_capacity(32);
            let mut s = HashMap::with_capacity(128);
            q.push_back((pos, 9));
            s.insert(pos, 1);
            while let Some((pos, height)) = q.pop_front() {
                let rating = *s.get(&pos).unwrap();
                if height > 0 {
                    for pos in self.neighbors(pos) {
                        let tile = self.0.get_mut(pos.0).unwrap().get_mut(pos.1).unwrap();
                        if tile.height + 1 == height {
                            let entry = s.entry(pos);
                            if matches!(entry, Entry::Vacant(_)) {
                                if height == 1 {
                                    tile.score += 1;
                                }
                                q.push_back((pos, tile.height));
                            }
                            if height == 1 {
                                tile.rating += rating;
                            }
                            *entry.or_default() += rating;
                        }
                    }
                }
            }
        }
    }

    fn data() -> Map {
        let map = read_by_line("tests/data/day10.input.txt")
            .map(|s| s.trim_end().as_bytes().iter().map(|b| b - b'0').collect())
            .collect();
        Map::new(map)
    }

    #[test]
    fn part1_and_2() {
        let map = data();
        let (score, rating) = map.trailheads().fold((0, 0), |(score, rating), tile| {
            (score + tile.score, rating + tile.rating)
        });
        assert_eq!(score, 816);
        assert_eq!(rating, 1960);
    }
}
