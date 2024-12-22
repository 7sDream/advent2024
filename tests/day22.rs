#[cfg(test)]
mod tests {
    use std::collections::{hash_map::Entry, HashMap, VecDeque};

    use advent2024::*;

    fn secrets(mut init: u64) -> impl Iterator<Item = u64> {
        Some(init).into_iter().chain(std::iter::from_fn(move || {
            init ^= init << 6;
            init &= 0xFFFFFF;

            init ^= init >> 5;
            init &= 0xFFFFFF;

            init ^= init << 11;
            init &= 0xFFFFFF;

            Some(init)
        }))
    }

    fn prices(init: u64) -> impl Iterator<Item = i8> {
        secrets(init).take(2000).map(|x| (x % 10) as i8)
    }

    #[derive(Debug, Clone)]
    struct Diff {
        diff: VecDeque<i8>,
        last: Option<i8>,
    }

    fn diffs(init: u64) -> impl Iterator<Item = Diff> {
        let mut diff = Diff {
            diff: VecDeque::with_capacity(4),
            last: None,
        };
        let mut prices = prices(init);
        std::iter::from_fn(move || {
            for current in prices.by_ref() {
                if let Some(last) = diff.last {
                    let d = current - last;
                    diff.last.replace(current);
                    if diff.diff.len() == 4 {
                        diff.diff.pop_front();
                    }
                    diff.diff.push_back(d);
                    if diff.diff.len() == 4 {
                        return Some(diff.clone());
                    }
                } else {
                    diff.last.replace(current);
                }
            }
            None
        })
    }

    fn bananas(init: u64) -> HashMap<VecDeque<i8>, i8> {
        let mut m = HashMap::<VecDeque<i8>, i8>::new();

        for diff in diffs(init) {
            if let Entry::Vacant(entry) = m.entry(diff.diff) {
                entry.insert(diff.last.unwrap());
            }
        }

        m
    }

    fn data(path: &str) -> impl Iterator<Item = u64> {
        read_by_line(path).map(|x| x.trim().parse().unwrap())
    }

    #[test]
    fn part1() {
        let result: u64 = data("tests/data/day22.input.txt")
            .map(|init| secrets(init).nth(2000).unwrap())
            .sum();

        assert_eq!(result, 15608699004)
    }

    #[test]
    fn part2() {
        let mut result = HashMap::<VecDeque<i8>, usize>::new();

        let m = data("tests/data/day22.input.txt")
            .map(bananas)
            .collect::<Vec<_>>();

        for record in &m {
            for diff in record.keys() {
                if let Entry::Vacant(entry) = result.entry(diff.clone()) {
                    entry.insert(
                        m.iter()
                            .filter_map(|record| record.get(diff))
                            .map(|price| *price as usize)
                            .sum::<usize>(),
                    );
                }
            }
        }

        let max = result.values().max().copied().unwrap();

        assert_eq!(max, 1791);
    }
}
