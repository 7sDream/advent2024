#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use advent2024::*;

    fn data() -> impl Iterator<Item = (u32, u32)> {
        read_by_line("tests/data/day1.input.txt").filter_map(|line| {
            let mut parts = line.split_ascii_whitespace().filter_map(|s| s.parse().ok());

            let (Some(n1), Some(n2)) = (parts.next(), parts.next()) else {
                return None;
            };

            Some((n1, n2))
        })
    }

    #[test]
    fn part1() {
        let mut l1: Vec<u32> = Vec::with_capacity(1024);
        let mut l2: Vec<u32> = Vec::with_capacity(1024);

        data().for_each(|(n1, n2)| {
            l1.push(n1);
            l2.push(n2);
        });

        l1.sort_unstable();
        l2.sort_unstable();

        let result: u32 = l1.into_iter().zip(l2).map(|(n1, n2)| n1.abs_diff(n2)).sum();

        assert_eq!(result, 2196996);
    }

    #[test]
    fn part2() {
        let mut m = HashMap::<u32, u32>::with_capacity(1024);

        let l1: Vec<u32> = data()
            .map(|(n1, n2)| {
                *m.entry(n2).or_default() += 1;
                n1
            })
            .collect();

        let result: u32 = l1
            .into_iter()
            .map(|n| n * m.get(&n).copied().unwrap_or_default())
            .sum();

        assert_eq!(result, 23655822);
    }
}
