#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use advent2024::*;

    type HostName = [char; 2];

    struct Graph {
        connections: HashMap<HostName, HashSet<HostName>>,
    }

    impl Graph {
        fn group_3(&self) -> HashSet<[&HostName; 3]> {
            let mut result = HashSet::new();

            for one in self.connections.keys() {
                let friends = self.connections.get(one).unwrap();
                for two in friends {
                    let friends2 = self.connections.get(two).unwrap();
                    let common = friends.intersection(friends2);
                    for three in common {
                        let mut group = [one, two, three];
                        group.sort();
                        result.insert(group);
                    }
                }
            }

            result
        }

        fn groups(&self) -> HashSet<Vec<HostName>> {
            // See https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
            fn finding(
                r: HashSet<HostName>,
                mut p: HashSet<HostName>,
                mut x: HashSet<HostName>,
                connections: &HashMap<HostName, HashSet<HostName>>,
                result: &mut HashSet<Vec<HostName>>,
            ) {
                if p.is_empty() && x.is_empty() {
                    if r.len() > 2 {
                        let mut rv: Vec<_> = r.into_iter().collect();
                        rv.sort();
                        result.insert(rv);
                    }
                } else {
                    for v in p.clone() {
                        let nv = connections.get(&v).unwrap();
                        finding(
                            r.iter().copied().chain(Some(v)).collect(),
                            p.intersection(nv).copied().collect(),
                            x.intersection(nv).copied().collect(),
                            connections,
                            result,
                        );
                        p.remove(&v);
                        x.insert(v);
                    }
                }
            }

            let mut result = HashSet::new();
            finding(
                HashSet::new(),
                self.connections.keys().copied().collect::<HashSet<_>>(),
                HashSet::new(),
                &self.connections,
                &mut result,
            );

            result
        }
    }

    fn data(path: &str) -> Graph {
        let iter = read_by_line(path)
            .map(|line| line.into_bytes())
            .map(|line| {
                (
                    [line[0] as char, line[1] as char],
                    [line[3] as char, line[4] as char],
                )
            });

        let mut graph = Graph {
            connections: HashMap::new(),
        };

        for (from, to) in iter {
            graph.connections.entry(from).or_default().insert(to);
            graph.connections.entry(to).or_default().insert(from);
        }

        graph
    }

    #[test]
    fn part1() {
        let graph = data("tests/data/day23.input.txt");
        let result = graph
            .group_3()
            .into_iter()
            .filter(|group| group.iter().any(|host| matches!(host, ['t', _])))
            .count();

        assert_eq!(result, 1240);
    }

    #[test]
    fn part2() {
        let graph = data("tests/data/day23.input.txt");
        let result = graph
            .groups()
            .into_iter()
            .max_by_key(|group| group.len())
            .unwrap();

        let txt: String = result
            .into_iter()
            .flat_map(|[a, b]| [',', a, b])
            .skip(1)
            .collect();

        assert_eq!(txt, "am,aq,by,ge,gf,ie,mr,mt,rw,sn,te,yi,zb");
    }
}
