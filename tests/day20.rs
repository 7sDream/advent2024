#[cfg(test)]
mod tests {
    use std::collections::{hash_map::Entry, HashMap, VecDeque};

    use advent2024::*;

    #[derive(Debug, Clone, Copy)]
    #[repr(u8)]
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

                let current_tile = self.map[pos.0][pos.1];

                match current_tile {
                    Tile::Empty => {
                        let neighbors = neighbors_limited(pos, self.size).into_iter().flatten();

                        for neighbor in neighbors {
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

        fn cheat_points(&self, start: Pos, max_jump: usize) -> Vec<(Pos, Pos, usize)> {
            let mut result = Vec::new();
            let max_jump_isize = max_jump as isize;

            for row in 0..self.size.0 {
                for col in 0..self.size.1 {
                    if matches!(self.map[row][col], Tile::Wall) {
                        continue;
                    }

                    let jump_start = (row, col);

                    let Some(l_half) = self.race(start, Some(jump_start), None).0 else {
                        continue;
                    };

                    let reachable = self.race(jump_start, None, Some(max_jump)).1;

                    for ox in -max_jump_isize..=max_jump_isize {
                        let y_offset_max = max_jump_isize - ox.abs();
                        for oy in -y_offset_max..=y_offset_max {
                            let jump_len = ox.unsigned_abs() + oy.unsigned_abs();
                            if jump_len != 0 {
                                let jump_end_y = row as isize + oy;
                                let jump_end_x = col as isize + ox;
                                if jump_end_x >= 0
                                    && jump_end_y >= 0
                                    && jump_end_y < self.size.0 as isize
                                    && jump_end_x < self.size.1 as isize
                                {
                                    let jump_end = (jump_end_y as usize, jump_end_x as usize);

                                    if matches!(self.map[jump_end.0][jump_end.1], Tile::Wall) {
                                        continue;
                                    }

                                    if reachable
                                        .get(&jump_end)
                                        .is_none_or(|score| jump_len < *score)
                                    {
                                        result.push((jump_start, jump_end, l_half + jump_len));
                                    }
                                }
                            }
                        }
                    }
                }
            }

            result
        }

        fn cheat_race(
            &self,
            start: Pos,
            end: Pos,
            max_jump: usize,
            no_cheat_score: usize,
        ) -> Vec<(Pos, Pos, usize)> {
            let mut to_end_scores = HashMap::<Pos, Option<usize>>::new();

            self.cheat_points(start, max_jump)
                .into_iter()
                .filter_map(|(jump_start, jump_end, left_half_score)| {
                    to_end_scores
                        .entry(jump_end)
                        .or_insert_with(|| self.race(jump_end, Some(end), None).0)
                        .and_then(|right_half| {
                            no_cheat_score.checked_sub(right_half + left_half_score)
                        })
                        .map(|saved| (jump_start, jump_end, saved))
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

    fn test(max_jump: usize) -> usize {
        let (start, end, maze) = data("tests/data/day20.input.txt");
        let no_cheat = maze.race(start, Some(end), None).0.unwrap();
        let cheats_saves = maze.cheat_race(start, end, max_jump, no_cheat);
        cheats_saves
            .into_iter()
            .filter(|(_, _, x)| *x >= 100)
            .count()
    }

    #[test]
    fn part1() {
        assert_eq!(test(2), 1286);
    }

    #[test]
    fn part2() {
        assert_eq!(test(20), 989316);
    }
}
