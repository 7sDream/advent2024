#[cfg(test)]
mod tests {
    use std::{
        cmp::Ordering,
        collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
        hash::Hash,
        sync::LazyLock,
    };

    use advent2024::*;

    trait Keyboard: Sized + Eq + Copy + Hash {
        const GAP: Self;

        fn mapping() -> Vec<Vec<Self>>;

        fn path_cost(path: &[Controller]) -> usize;

        fn path_cache() -> HashMap<(Self, Self), Vec<Controller>> {
            let m = Self::mapping();
            let size = (m.len(), m[0].len());
            let from_points = m.iter().enumerate().flat_map(|(row, line)| {
                line.iter()
                    .copied()
                    .enumerate()
                    .map(move |(col, button)| ((row, col), button))
            });

            let mut result: HashMap<(Self, Self), HashSet<Vec<Controller>>> = HashMap::new();

            for (pos, from_button) in from_points {
                let mut q: VecDeque<((usize, usize), Vec<Controller>)> = VecDeque::new();
                q.push_back((pos, vec![]));
                while let Some((pos, path)) = q.pop_front() {
                    let button = m[pos.0][pos.1];

                    if button == Self::GAP {
                        continue;
                    }

                    let entry = result.entry((from_button, button));

                    match entry {
                        Entry::Occupied(mut e) => {
                            match path.len().cmp(&e.get().iter().next().unwrap().len()) {
                                Ordering::Equal => {
                                    e.get_mut().insert(path.clone());
                                }
                                Ordering::Less => {
                                    unreachable!()
                                }
                                Ordering::Greater => {
                                    continue;
                                }
                            }
                        }
                        Entry::Vacant(e) => {
                            e.insert(Some(path.clone()).into_iter().collect());
                        }
                    }

                    for (idx, neighbor) in neighbors_limited(pos, size).into_iter().enumerate() {
                        let Some(neighbor) = neighbor else { continue };
                        let path = path.iter().copied().chain(Some(unsafe {
                            std::mem::transmute::<u8, Controller>(idx as u8)
                        }));
                        q.push_back((neighbor, path.collect()));
                    }
                }
            }

            result
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.into_iter()
                            .min_by_key(|path| Self::path_cost(path))
                            .unwrap(),
                    )
                })
                .collect()
        }
    }

    static NUM_PAD_CACHE: LazyLock<HashMap<(NumPad, NumPad), Vec<Controller>>> = LazyLock::new(NumPad::path_cache);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    enum NumPad {
        Zero,
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Active,
        Gap,
    }

    impl Keyboard for NumPad {
        const GAP: Self = Self::Gap;

        fn mapping() -> Vec<Vec<Self>> {
            vec![
                vec![Self::Seven, Self::Eight, Self::Nine],
                vec![Self::Four, Self::Five, Self::Six],
                vec![Self::One, Self::Two, Self::Three],
                vec![Self::Gap, Self::Zero, Self::Active],
            ]
        }

        fn path_cost(path: &[Controller]) -> usize {
            let mut path = path.to_owned();
            // 4 iter is enough to chose the most efficient move
            for _ in 0..4 {
                path = Controller::moves(Controller::Active, &path, Some(Controller::Active));
            }
            path.len()
        }
    }

    impl NumPad {
        pub fn one_move_times(from: Self, to: Self, middle_levels: usize) -> usize {
            let mut result = HashMap::new();
            let to = NUM_PAD_CACHE
                .get(&(from, to)).unwrap().iter().copied().chain(Some(Controller::Active));
            let from = Some(Controller::Active).into_iter().chain(to.clone());
            from.zip(to).for_each(|(f, t)| {
                *result.entry((f, t)).or_default() += 1;
            });

            for _ in 0..middle_levels {
                result = Controller::move_times(result);
            }

            result.values().sum()
        }

        fn move_times(start: Self, code: &[Self], middle_levels: usize) -> usize {
            let from = Some(&start).into_iter().chain(code.iter()).copied();
            let to = code.iter().copied();
            let points = from.zip(to);
            points
                .map(|(from, to)| Self::one_move_times(from, to, middle_levels))
                .sum()
        }
    }

    static CTRL_CACHE: LazyLock<HashMap<(Controller, Controller), Vec<Controller>>> = LazyLock::new(Controller::path_cache);

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(u8)]
    enum Controller {
        Left,
        Right,
        Up,
        Down,
        Active,
        Gap,
    }

    impl Keyboard for Controller {
        const GAP: Self = Self::Gap;

        fn mapping() -> Vec<Vec<Self>> {
            vec![
                vec![Self::GAP, Self::Up, Self::Active],
                vec![Self::Left, Self::Down, Self::Right],
            ]
        }
        
        // We don't need a cost function because the path_cache is hardcoded
        fn path_cost(_path: &[Controller]) -> usize {
            unimplemented!()
        }
        
        #[rustfmt::skip]
        fn path_cache() -> HashMap<(Self, Self), Vec<Controller>> {
            vec![
                ((Self::Active, Self::Active), vec![]),
                ((Self::Active, Self::Left), vec![Self::Down, Self::Left, Self::Left]),
                ((Self::Active, Self::Right), vec![Self::Down]),
                ((Self::Active, Self::Up), vec![Self::Left]),
                ((Self::Active, Self::Down), vec![Self::Left, Self::Down]),

                ((Self::Left, Self::Active), vec![Self::Right, Self::Right, Self::Up]),
                ((Self::Left, Self::Left), vec![]),
                ((Self::Left, Self::Right), vec![Self::Right, Self::Right]),
                ((Self::Left, Self::Up), vec![Self::Right, Self::Up]),
                ((Self::Left, Self::Down), vec![Self::Right]),
                
                ((Self::Right, Self::Active), vec![Self::Up]),
                ((Self::Right, Self::Left), vec![Self::Left, Self::Left]),
                ((Self::Right, Self::Right), vec![]),
                ((Self::Right, Self::Up), vec![Self::Left, Self::Up]),
                ((Self::Right, Self::Down), vec![Self::Left]),

                ((Self::Up, Self::Active), vec![Self::Right]),
                ((Self::Up, Self::Left), vec![Self::Down, Self::Left]),
                ((Self::Up, Self::Right), vec![Self::Down, Self::Right]),
                ((Self::Up, Self::Up), vec![]),
                ((Self::Up, Self::Down), vec![Self::Down]),

                ((Self::Down, Self::Active), vec![Self::Up, Self::Right]),
                ((Self::Down, Self::Left), vec![Self::Left]),
                ((Self::Down, Self::Right), vec![Self::Right]),
                ((Self::Down, Self::Up), vec![Self::Up]),
                ((Self::Down, Self::Down), vec![]),
            ].into_iter().collect()
        }
    }

    impl Controller {
        fn moves(start: Self, code: &[Self], end: Option<Self>) -> Vec<Controller> {
            let from = Some(&start).into_iter().chain(code.iter()).copied();
            let to = code.iter().copied().chain(end);
            let points = from.zip(to);
            points
                .flat_map(|(from, to)| {
                    CTRL_CACHE
                        .get(&(from, to))
                        .unwrap()
                        .iter()
                        .copied()
                        .chain(Some(Self::Active))
                })
                .collect()
        }

        fn move_times(code: HashMap<(Self, Self), usize>) -> HashMap<(Self, Self), usize> {
            let mut result = HashMap::new();

            for ((from, to), times) in code {
                let to = CTRL_CACHE
                    .get(&(from, to))
                    .unwrap()
                    .iter()
                    .copied()
                    .chain(Some(Self::Active));
                let from = Some(Self::Active).into_iter().chain(to.clone());
                from.zip(to).for_each(|(f, t)| {
                    *result.entry((f, t)).or_default() += times;
                });
            }

            result
        }
    }

    fn data(path: &str) -> impl Iterator<Item = (usize, Vec<NumPad>)> {
        read_by_line(path).map(|line| {
            (
                line[..line.len() - 2].parse().unwrap(),
                line.into_bytes()
                    .into_iter()
                    .filter_map(|c| match c {
                        b'0'..=b'9' => Some(c - b'0'),
                        b'A' => Some(10),
                        _ => None,
                    })
                    .map(|n| unsafe { std::mem::transmute::<u8, NumPad>(n) })
                    .collect::<Vec<_>>(),
            )
        })
    }

    #[test]
    fn part1() {
        let result: usize = data("tests/data/day21.input.txt")
            .map(|(num, code)| NumPad::move_times(NumPad::Active, &code, 2) * num)
            .sum();

        assert_eq!(result, 184716);
    }

    #[test]
    fn part2() {
        let result: usize = data("tests/data/day21.input.txt")
            .map(|(num, code)| NumPad::move_times(NumPad::Active, &code, 25) * num)
            .sum();

        assert_eq!(result, 229403562787554);
    }
}
