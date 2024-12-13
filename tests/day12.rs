#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet, VecDeque};

    use advent2024::*;

    #[derive(Debug)]
    struct Garden {
        width: usize,
        height: usize,
        plots: Vec<Vec<u8>>,
    }

    impl Garden {
        pub fn regions(self) -> GardenRegions {
            GardenRegions {
                checked: vec![vec![false; self.width]; self.height],
                garden: self,
                row: 0,
                col: 0,
            }
        }
    }

    struct GardenRegions {
        checked: Vec<Vec<bool>>,
        garden: Garden,
        row: usize,
        col: usize,
    }

    impl GardenRegions {
        fn move_to_next(&mut self) {
            self.col += 1;
            if self.col == self.garden.width {
                self.row += 1;
                self.col = 0;
            }
        }

        fn move_to_next_start(&mut self) {
            loop {
                self.move_to_next();
                if !self.current_checked() {
                    return;
                }
            }
        }

        fn current_checked(&self) -> bool {
            self.row < self.garden.height && self.checked[self.row][self.col]
        }

        fn current_plot(&self) -> u8 {
            self.garden.plots[self.row][self.col]
        }

        fn current_region(&self) -> Region {
            let plot = self.current_plot();

            let mut region = HashSet::new();
            let mut q = VecDeque::new();
            let mut s = HashSet::new();

            q.push_back((self.row, self.col));
            s.insert((self.row, self.col));
            while let Some((row, col)) = q.pop_front() {
                if self.garden.plots[row][col] == plot {
                    region.insert((row, col));
                    neighbors_limited((row, col), (self.garden.height, self.garden.width))
                        .into_iter()
                        .flatten()
                        .for_each(|pos| {
                            if !self.checked[pos.0][pos.1] && !s.contains(&pos) {
                                s.insert(pos);
                                q.push_back(pos);
                            }
                        });
                }
            }

            Region { plots: region }
        }
    }

    impl Iterator for GardenRegions {
        type Item = Region;

        fn next(&mut self) -> Option<Self::Item> {
            if self.row >= self.garden.height {
                return None;
            }

            let region = self.current_region();

            for (row, col) in region.plots.iter().copied() {
                self.checked[row][col] = true;
            }

            self.move_to_next_start();

            Some(region)
        }
    }

    struct Region {
        plots: HashSet<(usize, usize)>,
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    #[repr(usize)]
    enum SideDir {
        Left,
        Right,
        Up,
        Down,
    }

    #[derive(Debug)]
    struct Fence(usize, usize, SideDir);

    impl Region {
        pub fn area(&self) -> usize {
            self.plots.len()
        }

        pub fn perimeter(&self) -> usize {
            self.fences().count()
        }

        fn check_fence(
            &self,
            (row, col): (usize, usize),
            dir: usize,
            neighbor: Option<(usize, usize)>,
        ) -> Option<Fence> {
            let need_fence = neighbor
                .as_ref()
                .map(|x| !self.plots.contains(x))
                .unwrap_or(true);

            need_fence.then(|| {
                let dir = match dir {
                    0 => SideDir::Left,
                    1 => SideDir::Right,
                    2 => SideDir::Up,
                    3 => SideDir::Down,
                    _ => unreachable!(),
                };
                Fence(row, col, dir)
            })
        }

        pub fn fences(&self) -> impl Iterator<Item = Fence> + '_ {
            self.plots.iter().flat_map(|plot| {
                neighbors(*plot)
                    .into_iter()
                    .enumerate()
                    .filter_map(|(dir, neighbor)| self.check_fence(*plot, dir, neighbor))
            })
        }

        pub fn side_count(&self) -> usize {
            let mut fences_by_dir: [HashMap<usize, Vec<usize>>; 4] =
                std::array::from_fn(|_| HashMap::new());

            self.fences().for_each(|Fence(row, col, dir)| match dir {
                SideDir::Left | SideDir::Right => fences_by_dir[dir as usize]
                    .entry(col)
                    .or_default()
                    .push(row),
                SideDir::Up | SideDir::Down => fences_by_dir[dir as usize]
                    .entry(row)
                    .or_default()
                    .push(col),
            });

            fn line_side_count(mut ns: Vec<usize>) -> usize {
                ns.sort();
                let mut count = 1;
                ns.into_iter().reduce(|x, y| {
                    if x + 1 != y {
                        count += 1;
                    }
                    y
                });
                count
            }

            fn dir_side_count(lines: impl IntoIterator<Item = Vec<usize>>) -> usize {
                lines.into_iter().map(line_side_count).sum()
            }

            fences_by_dir
                .into_iter()
                .map(|dir_fences| dir_side_count(dir_fences.into_values()))
                .sum()
        }
    }

    fn data() -> Garden {
        let plots = read_by_line("tests/data/day12.input.txt")
            .map(|line| {
                let mut line = line.into_bytes();
                line.pop(); // remove \n
                line
            })
            .collect::<Vec<_>>();

        Garden {
            width: plots.first().unwrap().len(),
            height: plots.len(),
            plots,
        }
    }

    #[test]
    fn part1() {
        let garden = data();
        let result: usize = garden.regions().map(|x| x.area() * x.perimeter()).sum();

        assert_eq!(result, 1431316)
    }

    #[test]
    fn part2() {
        let garden = data();
        let result: usize = garden.regions().map(|x| x.area() * x.side_count()).sum();

        assert_eq!(result, 821428)
    }
}
