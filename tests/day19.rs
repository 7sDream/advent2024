#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        collections::{HashMap, HashSet},
    };

    use advent2024::*;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Pattern(String);

    #[derive(Debug)]
    struct Patterns(HashSet<Pattern>);

    impl Patterns {
        pub fn check(&self, logo: &str) -> usize {
            thread_local! {
                static CACHE: RefCell<HashMap<String, usize>> = RefCell::default();
            }

            if logo.is_empty() {
                1
            } else {
                let cr = CACHE.with_borrow(|cache| cache.get(logo).copied());
                if let Some(r) = cr {
                    r
                } else {
                    let ql = logo.len();
                    let result = self
                        .0
                        .iter()
                        .filter_map(|x| {
                            let sp = ql.checked_sub(x.0.len());
                            sp.filter(|sp| logo[*sp..] == x.0)
                        })
                        .map(|sp| self.check(&logo[..sp]))
                        .sum();
                    CACHE.with_borrow_mut(|cache| cache.insert(logo.to_owned(), result));
                    result
                }
            }
        }
    }

    impl FromIterator<Pattern> for Patterns {
        fn from_iter<T: IntoIterator<Item = Pattern>>(iter: T) -> Self {
            Self(iter.into_iter().collect())
        }
    }

    fn data(path: &str) -> (Patterns, impl Iterator<Item = String>) {
        let mut lines = read_by_line(path);
        let one = lines.next().unwrap();
        let patterns = one
            .split(',')
            .map(|s| Pattern(s.trim().to_owned()))
            .collect();

        lines.next(); // skip empty line

        (
            patterns,
            lines.map(|mut l| {
                l.pop();
                l
            }),
        )
    }

    #[test]
    fn part1() {
        let (patterns, logos) = data("tests/data/day19.input.txt");
        let result = logos
            .filter(|logo| {
                let is = patterns.check(logo);
                is > 0
            })
            .count();
        assert_eq!(result, 319);
    }

    #[test]
    fn part2() {
        let (patterns, logos) = data("tests/data/day19.input.txt");
        let result: usize = logos.map(|logo| patterns.check(&logo)).sum();
        assert_eq!(result, 692575723305545);
    }
}
