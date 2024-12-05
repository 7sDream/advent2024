#[cfg(test)]
mod tests {
    use std::{
        cmp::Ordering,
        collections::{HashMap, HashSet},
    };

    use advent2024::*;

    #[derive(Debug, Default)]
    struct Rule(HashMap<u32, HashSet<u32>>);
    impl Rule {
        fn check(&self, before: &u32, after: &u32) -> bool {
            if let Some(afters) = self.0.get(after) {
                !afters.contains(before)
            } else {
                true
            }
        }

        fn cmp(&self, a: &u32, b: &u32) -> Ordering {
            if a == b {
                return Ordering::Equal;
            }

            if let Some(afters) = self.0.get(b) {
                if afters.contains(a) {
                    return Ordering::Greater;
                }
            }

            Ordering::Less
        }
    }

    impl FromIterator<(u32, u32)> for Rule {
        fn from_iter<T: IntoIterator<Item = (u32, u32)>>(iter: T) -> Self {
            let mut rule = Self::default();
            for (prev, next) in iter {
                rule.0.entry(prev).or_default().insert(next);
            }
            rule
        }
    }

    fn data() -> (Rule, Vec<Vec<u32>>) {
        let mut lines = read_by_line("tests/data/day5.input.txt");

        let rule = lines
            .by_ref()
            .take_while(|line| line != "\n")
            .map(|line| {
                let mut parts = line.trim_end().splitn(2, '|');
                (
                    parts.next().unwrap().parse::<u32>().unwrap(),
                    parts.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .collect();

        let updates = lines
            .map(|line| {
                line.trim_end()
                    .split(',')
                    .map(|page| page.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        (rule, updates)
    }

    #[test]
    fn part1() {
        let (rule, updates) = data();

        let result = updates
            .into_iter()
            .filter(|update| update.is_sorted_by(|a, b| rule.check(a, b)))
            .map(|update| update[update.len() / 2])
            .sum::<u32>();

        assert_eq!(result, 4905);
    }

    #[test]
    fn part2() {
        let (rule, updates) = data();

        let result = updates
            .into_iter()
            .filter(|update| !update.is_sorted_by(|a, b| rule.check(a, b)))
            .map(|mut update| {
                update.sort_unstable_by(|a, b| rule.cmp(a, b));
                update[update.len() / 2]
            })
            .sum::<u32>();

        assert_eq!(result, 6204);
    }
}
