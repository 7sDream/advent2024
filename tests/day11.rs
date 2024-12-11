#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::HashMap};

    use advent2024::*;

    fn data() -> Vec<u64> {
        read_by_line("tests/data/day11.input.txt")
            .next()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect()
    }

    // If uses rayon or other multi-thread calculation
    // need change this to LazyCell<Mutex<HashMap<...>>>
    thread_local! {
        static CACHE: RefCell<HashMap<(u64, usize), usize>> = RefCell::default();
    }

    fn f(x: u64, n: usize) -> usize {
        if n == 0 {
            return 1;
        }

        if let Some(value) = CACHE.with_borrow(|cache| cache.get(&(x, n)).copied()) {
            return value;
        }

        let result = if x == 0 {
            // Rule 1
            f(1, n - 1)
        } else {
            let s = x.to_string();
            let l = s.len();
            if l % 2 == 0 {
                // Rule 2
                f(s[0..l / 2].parse().unwrap(), n - 1) + f(s[l / 2..].parse().unwrap(), n - 1)
            } else {
                // Rule 3
                f(x * 2024, n - 1)
            }
        };

        CACHE.with_borrow_mut(|cache| {
            cache.insert((x, n), result);
        });

        result
    }

    #[test]
    fn part1() {
        let count: usize = data().into_iter().map(|x| f(x, 25)).sum();
        assert_eq!(count, 183620);
    }

    #[test]
    fn part2() {
        let count: usize = data().into_iter().map(|x| f(x, 75)).sum();
        assert_eq!(count, 220377651399268);
    }
}
