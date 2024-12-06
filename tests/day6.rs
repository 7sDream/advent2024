#[cfg(test)]
mod tests {
    use advent2024::*;

    static DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    #[repr(u8)]
    #[derive(Clone)]
    enum Block {
        // u8 for recoding all dirs when step in this block
        // 76543210
        // ....LDRU
        Empty(u8),
        Obstruction,
    }

    enum WalkStep {
        At(usize, usize),
        Out,
        Loop,
    }

    #[derive(Clone)]
    struct Map(Vec<Vec<Block>>);

    // For debuging, not important
    impl core::fmt::Debug for Map {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use std::fmt::Write;

            for line in self.0.iter() {
                for block in line.iter() {
                    let c = match block {
                        Block::Empty(0) => '.',
                        Block::Empty(x) => {
                            if x & 0b1010 != 0 {
                                if x & 0b0101 != 0 {
                                    '+'
                                } else {
                                    '-'
                                }
                            } else {
                                '|'
                            }
                        }
                        Block::Obstruction => '#',
                    };
                    f.write_char(c)?;
                }
                f.write_char('\n')?;
            }
            Ok(())
        }
    }

    impl Map {
        pub fn positions(&self) -> impl Iterator<Item = (usize, usize, &'_ Block)> + '_ {
            self.0
                .iter()
                .enumerate()
                .flat_map(|(row, line)| line.iter().enumerate().map(move |(col, c)| (row, col, c)))
        }

        pub fn visited_block_count(&self) -> usize {
            self.positions()
                .filter(|(_, _, x)| matches!(x, Block::Empty(1..)))
                .count()
        }

        pub fn guard_walk(
            &mut self,
            mut guard: (usize, usize),
        ) -> impl Iterator<Item = WalkStep> + '_ {
            let mut dir_idx = 0;
            let mut end = false;

            std::iter::from_fn(move || loop {
                if end {
                    return None;
                }

                let (oy, ox) = DIRECTIONS[dir_idx];
                let Some(row) = (guard.0 as isize + oy)
                    .try_into()
                    .ok()
                    .filter(|x| (0..self.0.len()).contains(x))
                else {
                    end = true;
                    return Some(WalkStep::Out);
                };
                let line = self.0.get_mut(row).unwrap();
                let Some(col) = (guard.1 as isize + ox)
                    .try_into()
                    .ok()
                    .filter(|x| (0..line.len()).contains(x))
                else {
                    end = true;
                    return Some(WalkStep::Out);
                };
                let block = line.get_mut(col).unwrap();

                match block {
                    Block::Empty(ref mut dirs) => {
                        if *dirs & (1 << dir_idx) != 0 {
                            end = true;
                            return Some(WalkStep::Loop);
                        }
                        guard = (row, col);
                        *dirs |= 1 << dir_idx;
                        return Some(WalkStep::At(row, col));
                    }
                    Block::Obstruction => {
                        dir_idx = (dir_idx + 1) % 4;
                        if let Block::Empty(ref mut x) = self.0[guard.0][guard.1] {
                            *x |= 1 << dir_idx;
                        }
                        continue;
                    }
                }
            })
        }
    }

    fn data() -> (Map, (usize, usize)) {
        let mut guard = (0, 0);
        let map = read_by_line("tests/data/day6.input.txt")
            .enumerate()
            .map(|(row, line)| {
                line.trim_end()
                    .chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        '.' => Block::Empty(0),
                        '^' => {
                            guard = (row, col);
                            Block::Empty(1)
                        }
                        '#' => Block::Obstruction,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();

        (Map(map), guard)
    }

    #[test]
    fn part1() {
        let (mut map, guard) = data();
        map.guard_walk(guard).for_each(drop);
        assert_eq!(map.visited_block_count(), 5162);
    }

    #[test]
    fn part2() {
        let (map, guard) = data();
        let positions = map
            .positions()
            .filter(|(_, _, block)| matches!(block, Block::Empty(0)));

        let result = positions
            .filter(|(y, x, _)| {
                let mut map = map.clone();
                map.0[*y][*x] = Block::Obstruction;
                matches!(map.guard_walk(guard).last().unwrap(), WalkStep::Loop)
            })
            .count();

        assert_eq!(result, 1909);
    }
}
