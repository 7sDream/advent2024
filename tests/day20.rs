#[cfg(test)]
mod tests {
    use std::collections::{hash_map::Entry, HashMap, VecDeque};

    use advent2024::*;

    #[derive(Debug, Clone, Copy)]
    enum Tile {
        Empty,
        Wall,
    }

    type Pos = (usize, usize);

    struct Maze {
        size: Pos,
        map: Vec<Vec<Tile>>,
    }

    impl Maze {
        pub fn race(
            &self,
            start: Pos,
            end: Option<Pos>,
            max: Option<usize>,
        ) -> (Option<usize>, HashMap<Pos, usize>) {
            let mut s = HashMap::<Pos, usize>::new();
            let mut q = VecDeque::<(Pos, usize)>::new();

            s.insert(start, 0);
            q.push_back((start, 0));

            while let Some((pos, score)) = q.pop_front() {
                if max.is_some_and(|max| score > max) {
                    continue;
                }
                if end.is_some_and(|end| pos == end) {
                    return (Some(score), s);
                }

                let tile = self.map[pos.0][pos.1];

                match tile {
                    Tile::Empty => {
                        for neighbor in neighbors_limited(pos, self.size).into_iter().flatten() {
                            let to_tile = self.map[neighbor.0][neighbor.1];
                            let entry = s.entry(neighbor);
                            if matches!(to_tile, Tile::Wall) || matches!(entry, Entry::Occupied(_))
                            {
                                continue;
                            }
                            entry.insert_entry(score + 1);
                            q.push_back((neighbor, score + 1));
                        }
                    }
                    Tile::Wall => unreachable!(),
                }
            }

            (None, s)
        }

        fn cheat_points(&self, start: Pos, max_cheat: usize) -> Vec<(Pos, Pos, usize)> {
            let mut result = Vec::new();
            let max_cheat_isize = max_cheat as isize;

            for row in 0..self.size.0 {
                for col in 0..self.size.1 {
                    if matches!(self.map[row][col], Tile::Wall) {
                        continue;
                    }

                    let cheat_start = (row, col);
                    let Some(start_to_cheat_start) = self.race(start, Some(cheat_start), None).0
                    else {
                        continue;
                    };

                    let reachable = self.race(cheat_start, None, Some(max_cheat)).1;

                    for ox in -max_cheat_isize..=max_cheat_isize {
                        let y_offset_max = max_cheat_isize - ox.abs();
                        for oy in -y_offset_max..=y_offset_max {
                            let cheat_len = ox.unsigned_abs() + oy.unsigned_abs();
                            if cheat_len != 0 {
                                let cheat_end_y = row as isize + oy;
                                let cheat_end_x = col as isize + ox;
                                if cheat_end_x >= 0
                                    && cheat_end_y >= 0
                                    && cheat_end_y < self.size.0 as isize
                                    && cheat_end_x < self.size.1 as isize
                                {
                                    let cheat_end = (cheat_end_y as usize, cheat_end_x as usize);

                                    if matches!(self.map[cheat_end.0][cheat_end.1], Tile::Wall) {
                                        continue;
                                    }

                                    if reachable
                                        .get(&cheat_end)
                                        .is_none_or(|score| cheat_len < *score)
                                    {
                                        result.push((
                                            cheat_start,
                                            cheat_end,
                                            start_to_cheat_start + cheat_len,
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            result
        }

        fn cheat_saves(
            &self,
            start: Pos,
            end: Pos,
            max_cheat: usize,
            no_cheat_score: usize,
        ) -> Vec<(Pos, Pos, usize)> {
            let mut to_end_scores = HashMap::<Pos, Option<usize>>::new();

            self.cheat_points(start, max_cheat)
                .into_iter()
                .filter_map(|(cheat_start, cheat_end, start_to_cheat_end_score)| {
                    to_end_scores
                        .entry(cheat_end)
                        .or_insert_with(|| self.race(cheat_end, Some(end), None).0)
                        .and_then(|cheat_end_to_end_score| {
                            no_cheat_score
                                .checked_sub(start_to_cheat_end_score + cheat_end_to_end_score)
                        })
                        .map(|saved| (cheat_start, cheat_end, saved))
                })
                .collect()
        }
    }

    fn data(path: &str) -> (Pos, Pos, Maze) {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map: Vec<_> = read_by_line(path)
            .enumerate()
            .map(|(row, line)| {
                line.into_bytes()
                    .into_iter()
                    .filter(|b| !matches!(b, b'\r' | b'\n'))
                    .enumerate()
                    .map(|(col, b)| match b {
                        b'#' => Tile::Wall,
                        b'.' => Tile::Empty,
                        b'S' => {
                            start = (row, col);
                            Tile::Empty
                        }
                        b'E' => {
                            end = (row, col);
                            Tile::Empty
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        (
            start,
            end,
            Maze {
                size: (map.len(), map.first().unwrap().len()),
                map,
            },
        )
    }

    fn cheat_save_more_then_100(max_cheat: usize) -> usize {
        let (start, end, maze) = data("tests/data/day20.input.txt");
        let no_cheat = maze.race(start, Some(end), None).0.unwrap();
        let cheats_saves = maze.cheat_saves(start, end, max_cheat, no_cheat);
        cheats_saves
            .into_iter()
            .filter(|(_, _, x)| *x >= 100)
            .count()
    }

    #[test]
    fn part1() {
        assert_eq!(cheat_save_more_then_100(2), 1286);
    }

    #[test]
    fn part2() {
        assert_eq!(cheat_save_more_then_100(20), 989316);
    }
}
