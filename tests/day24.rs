#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        str::FromStr,
    };

    use advent2024::*;

    #[derive(Debug, Clone, Copy)]
    enum LogicGate {
        And,
        Or,
        Xor,
    }

    impl FromStr for LogicGate {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "AND" => Self::And,
                "OR" => Self::Or,
                "XOR" => Self::Xor,
                _ => Err(())?,
            })
        }
    }

    impl LogicGate {
        pub fn logic(&self, lhs: bool, rhs: bool) -> bool {
            match self {
                Self::And => lhs && rhs,
                Self::Or => lhs || rhs,
                Self::Xor => lhs != rhs,
            }
        }
    }

    fn bits(x: u64) -> impl Iterator<Item = bool> {
        (0..u64::BITS).map(move |n| (x >> n) & 1 != 0)
    }

    #[derive(Debug, Clone)]
    struct Device {
        bit_count: usize,
        wires: HashMap<String, bool>,
        connections: HashMap<String, (String, LogicGate, String)>,
    }

    impl Device {
        fn get_impl(&mut self, wire: String, circle_check: &mut HashSet<String>) -> Option<bool> {
            let entry = self.wires.get(&wire).copied();
            match entry {
                Some(_) => entry,
                None => {
                    if circle_check.contains(&wire) {
                        return None;
                    }
                    let (lhs, gate, rhs) = self.connections.get(&wire).cloned()?;

                    circle_check.insert(wire.clone());
                    let result = gate.logic(
                        self.get_impl(lhs, circle_check)?,
                        self.get_impl(rhs, circle_check)?,
                    );
                    circle_check.remove(&wire);
                    self.wires.insert(wire, result);
                    Some(result)
                }
            }
        }

        fn get(&mut self, wire: String) -> Option<bool> {
            self.get_impl(wire, &mut HashSet::new())
        }

        fn output(&mut self) -> Option<Vec<bool>> {
            (0..=99)
                .map(|x| format!("z{:02}", x))
                .take(self.bit_count)
                .map(|wire| self.get(wire))
                .collect()
        }

        fn output_number(&mut self) -> Option<u64> {
            self.output().map(|output| {
                output
                    .into_iter()
                    .enumerate()
                    .filter(|(_, value)| *value)
                    .fold(0, |acc, (bit, _)| acc | (1 << bit))
            })
        }

        fn fill_input(&mut self, prefix: &str, value: u64) {
            bits(value)
                .take(self.bit_count)
                .enumerate()
                .for_each(|(pos, value)| {
                    let name = format!("{}{:02}", prefix, pos);
                    self.wires.insert(name, value);
                });
        }

        fn error_bits(&mut self, x: u64, y: u64, z: u64) -> Option<Vec<usize>> {
            self.wires.clear();
            self.fill_input("x", x);
            self.fill_input("y", y);
            Some(
                bits(self.output_number()?)
                    .zip(bits(z))
                    .enumerate()
                    .filter(|(_, (out, expected))| out != expected)
                    .map(|(bit, _)| bit)
                    .collect(),
            )
        }

        fn all_wire_names(&self) -> impl Iterator<Item = String> + use<'_> {
            self.connections
                .iter()
                .flat_map(|(output, (left, _, right))| [output, left, right])
                .cloned()
        }

        fn all_middle_wire_names(&self) -> HashSet<String> {
            self.all_wire_names()
                .filter(|x| !(x.starts_with('x') || x.starts_with('y') || x.starts_with('z')))
                .collect()
        }

        fn dependencies(&self, name: String) -> HashSet<String> {
            let mut result = HashSet::<String>::new();
            let mut q = VecDeque::new();
            q.push_back(name.clone());
            result.insert(name);

            while let Some(name) = q.pop_front() {
                if name.starts_with('x') || name.starts_with('y') {
                    result.remove(&name);
                    continue;
                }

                if let Some((left, _, right)) = self.connections.get(&name) {
                    if !result.contains(left) {
                        result.insert(left.clone());
                        q.push_back(left.clone());
                    }
                    if !result.contains(right) {
                        result.insert(right.clone());
                        q.push_back(right.clone());
                    }
                }
            }

            result
        }

        fn dependencies_zbit(&self, bit: usize) -> HashSet<String> {
            self.dependencies(format!("z{:02}", bit))
        }

        fn swap(&mut self, l: impl AsRef<str>, r: impl AsRef<str>) {
            let lc = self.connections.get(l.as_ref()).unwrap().clone();
            let rc = self.connections.get(r.as_ref()).unwrap().clone();
            self.connections.insert(l.as_ref().to_string(), rc);
            self.connections.insert(r.as_ref().to_string(), lc);
        }
    }

    fn data(path: &str) -> Device {
        let mut iter = read_by_line(path);

        let wires: HashMap<_, _> = iter
            .by_ref()
            .take_while(|s| !s.trim().is_empty())
            .map(|line| {
                let mut parts = line.trim().splitn(2, ": ");
                let wire = parts.next().unwrap().to_string();
                let value = matches!(parts.next().unwrap(), "1");
                (wire, value)
            })
            .collect();

        let connections = iter
            .map(|line| {
                let mut parts = line.trim().split(' ');
                let l = parts.next().unwrap().to_string();
                let gate = parts.next().unwrap().parse::<LogicGate>().unwrap();
                let r = parts.next().unwrap().to_string();
                let _ = parts.next().unwrap();
                let output = parts.next().unwrap().to_string();
                (output, (l, gate, r))
            })
            .collect();

        let max_bit = wires
            .keys()
            .filter_map(|name| name[1..].parse::<usize>().ok())
            .max()
            .unwrap()
            + 1;

        Device {
            bit_count: max_bit,
            wires,
            connections,
        }
    }

    #[test]
    fn part1() {
        let mut device = data("tests/data/day24.input.txt");
        assert_eq!(device.output_number().unwrap(), 46362252142374);
    }

    fn check(
        device: &mut Device,
        checks: impl IntoIterator<Item = usize>,
        output: bool,
    ) -> HashMap<usize, HashSet<String>> {
        let candidates = if output {
            device.all_middle_wire_names()
        } else {
            HashSet::new()
        };
        let mut result = HashMap::new();
        for bit in checks {
            let mut prefect = true;
            let mut candidates = candidates.clone();
            for abc in 0..8 {
                let [c, a, b] = bits(abc)
                    .take(3)
                    .collect::<Vec<_>>()
                    .first_chunk()
                    .copied()
                    .unwrap();

                let mut x: u64 = if a { 1 } else { 0 } << bit;
                let mut y: u64 = if b { 1 } else { 0 } << bit;
                if c && bit > 0 {
                    x |= 1 << (bit - 1);
                    y |= 1 << (bit - 1);
                }

                let z = x.overflowing_add(y).0;
                let target = (z >> bit) & 1 != 0;

                if device.error_bits(x, y, z).is_none_or(|v| v.contains(&bit)) {
                    prefect = false;
                }

                if output {
                    candidates = candidates
                        .intersection(
                            &device
                                .wires
                                .iter()
                                .filter(|(_, value)| **value == target)
                                .map(|(name, _)| name)
                                .cloned()
                                .collect::<HashSet<_>>(),
                        )
                        .cloned()
                        .collect();
                }
            }
            if !prefect {
                if output {
                    println!("z{:02} possible origin wires: {:?}", bit, candidates);
                }
                result.insert(bit, candidates);
            }
        }

        result
    }

    // Part 2 is done by guessing, So this code may not usable for your input.
    // I'm lucky enough that there are three output wires(zxx) is swapped,
    // If not the case, the brute-force will take forever...
    #[test]
    fn part2() {
        let mut device = data("tests/data/day24.input.txt");
        let full_bits = 0..device.bit_count;
        let mut swapped = vec![];

        println!("===== guess there are some output wire is swapped =====");
        for (bit, wires) in check(&mut device, full_bits.clone(), true) {
            if wires.len() == 1 {
                let a = format!("z{:02}", bit);
                let b = wires.into_iter().next().unwrap();
                device.swap(&a, &b);
                swapped.extend([a, b]);
            }
        }

        println!("===== brute force the remaining =====");
        let error_bits = check(&mut device, full_bits.clone(), false);
        let candidates = error_bits
            .keys()
            .map(|bit| device.dependencies_zbit(*bit))
            .fold(HashSet::new(), |acc, curr| {
                acc.union(&curr).cloned().collect()
            });

        'outer: for l in candidates.iter() {
            for r in candidates.iter() {
                let mut tmp_device = device.clone();
                tmp_device.swap(l, r);
                if check(&mut tmp_device, error_bits.keys().copied(), false).is_empty() {
                    println!("Maybe answer: {} {}", l, r);
                    if check(&mut tmp_device, full_bits.clone(), false).is_empty() {
                        println!("Yes, it is");
                        device = tmp_device;
                        swapped.extend([l.to_string(), r.to_string()]);
                        break 'outer;
                    }
                }
            }
        }

        // Check
        assert!(check(&mut device, full_bits.clone(), false).is_empty());
        // So the answer is
        swapped.sort();
        println!("Swapped wires: {:?}", swapped);

        let answer = swapped
            .iter()
            .flat_map(|wire| [",", wire])
            .skip(1)
            .collect::<String>();
        assert_eq!(answer, "cbd,gmh,jmq,qrh,rqf,z06,z13,z38")
    }
}
