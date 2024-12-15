#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use advent2024::*;

    #[derive(Debug, Clone, Copy)]
    enum Tile {
        Empty,
        Wall,
        BoxLeft,
        BoxRight,
    }

    #[derive(Debug, Clone, Copy)]
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    impl Direction {
        fn offset(&self) -> (isize, isize) {
            match self {
                Self::Left => (0, -1),
                Self::Right => (0, 1),
                Self::Up => (-1, 0),
                Self::Down => (1, 0),
            }
        }
    }

    #[derive(Debug)]
    struct Robot {
        position: (usize, usize),
    }

    impl Robot {
        fn go(&mut self, map: &mut [Vec<Tile>], dir: Direction) -> Option<usize> {
            if let Some(boxes) = self.moved_boxes(map, dir) {
                let (oy, ox) = dir.offset();
                boxes.keys().copied().for_each(|(y, x)| {
                    map[y][x] = Tile::Empty;
                });
                boxes.iter().for_each(|((y, x), tile)| {
                    map[(*y as isize + oy) as usize][(*x as isize + ox) as usize] = *tile;
                });

                map[self.position.0][self.position.1] = Tile::Empty;
                self.position = (
                    (self.position.0 as isize + oy) as usize,
                    (self.position.1 as isize + ox) as usize,
                );

                Some(boxes.len())
            } else {
                None
            }
        }

        fn moved_boxes(
            &self,
            map: &[Vec<Tile>],
            dir: Direction,
        ) -> Option<HashMap<(usize, usize), Tile>> {
            let (oy, ox) = dir.offset();
            let mut boxes: HashMap<(usize, usize), Tile> = HashMap::new();
            let mut boxes_count = 0;
            let mut checks = HashSet::new();
            checks.insert(self.position);
            loop {
                checks = checks
                    .into_iter()
                    .map(|(y, x)| ((y as isize + oy) as usize, (x as isize + ox) as usize))
                    .collect();

                if checks
                    .iter()
                    .any(|(y, x)| matches!(map[*y][*x], Tile::Wall))
                {
                    return None;
                }

                checks = checks
                    .iter()
                    .copied()
                    .flat_map(|(y, x)| match map[y][x] {
                        Tile::BoxLeft => {
                            boxes.insert((y, x), Tile::BoxLeft);
                            if matches!(map[y][x + 1], Tile::BoxRight) {
                                boxes.insert((y, x + 1), Tile::BoxRight);
                                [Some((y, x)), Some((y, x + 1))]
                            } else {
                                [Some((y, x)), None]
                            }
                        }
                        Tile::BoxRight => {
                            boxes.insert((y, x), Tile::BoxRight);
                            if matches!(map[y][x - 1], Tile::BoxLeft) {
                                boxes.insert((y, x - 1), Tile::BoxLeft);
                                [Some((y, x)), Some((y, x - 1))]
                            } else {
                                [Some((y, x)), None]
                            }
                        }
                        _ => [None, None],
                    })
                    .flatten()
                    .collect();

                if boxes.len() == boxes_count {
                    return Some(boxes);
                }

                boxes_count = boxes.len();
            }
        }
    }

    #[derive(Debug)]
    struct Warehouse {
        robot: Robot,
        map: Vec<Vec<Tile>>,
    }

    impl Warehouse {
        fn robot_move(&mut self, dir: Direction) -> Option<usize> {
            self.robot.go(&mut self.map, dir)
        }

        fn gps(&self) -> usize {
            self.map
                .iter()
                .enumerate()
                .flat_map(|(row, line)| {
                    line.iter()
                        .enumerate()
                        .map(move |(col, tile)| (row, col, *tile))
                })
                .filter(|(_, _, tile)| matches!(tile, Tile::BoxLeft))
                .map(|(row, col, _)| row * 100 + col)
                .sum()
        }
    }

    fn data(path: &str, double: bool) -> (Warehouse, impl Iterator<Item = Direction>) {
        let mut lines = read_by_line(path);

        let m = lines.by_ref().take_while(|x| !x.trim_end().is_empty());

        let mut robot = (0, 0);
        let map: Vec<Vec<_>> = m
            .enumerate()
            .map(|(row, line)| {
                line.trim_end()
                    .as_bytes()
                    .iter()
                    .enumerate()
                    .flat_map(|(col, tile)| {
                        let tile = match tile {
                            b'.' => Tile::Empty,
                            b'#' => Tile::Wall,
                            b'O' => Tile::BoxLeft,
                            b'@' => {
                                robot = (row, col);
                                Tile::Empty
                            }
                            _ => unreachable!(),
                        };
                        Some(tile).into_iter().chain(if double {
                            Some(if matches!(tile, Tile::BoxLeft) {
                                Tile::BoxRight
                            } else {
                                tile
                            })
                        } else {
                            None
                        })
                    })
                    .collect()
            })
            .collect();

        let warehouse = Warehouse {
            robot: Robot {
                position: if double {
                    (robot.0, robot.1 * 2)
                } else {
                    robot
                },
            },
            map,
        };

        let movements = lines
            .flat_map(|line| line.into_bytes())
            .filter(|b| !matches!(b, b'\r' | b'\n'))
            .map(|b| match b {
                b'<' => Direction::Left,
                b'>' => Direction::Right,
                b'^' => Direction::Up,
                b'v' => Direction::Down,
                byte => unreachable!("unknown byte {}", byte),
            });

        (warehouse, movements)
    }

    #[test]
    fn part1() {
        let (mut warehouse, movement) = data("tests/data/day15.input.txt", false);
        movement.for_each(|dir| {
            warehouse.robot_move(dir);
        });

        assert_eq!(warehouse.gps(), 1492518);
    }

    #[test]
    fn part2() {
        let (mut warehouse, movement) = data("tests/data/day15.input.txt", true);
        movement.for_each(|dir| {
            warehouse.robot_move(dir);
        });
        assert_eq!(warehouse.gps(), 1512860);
    }
}
