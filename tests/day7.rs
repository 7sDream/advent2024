#[cfg(test)]
mod tests {
    use std::ops::ControlFlow;

    use advent2024::*;

    struct Equ {
        result: i64,
        numbers: Vec<i64>,
    }

    struct SolveStep<'a> {
        result: i64,
        numbers: &'a [i64],
    }

    trait Operator {
        fn rollback(&self, result: i64, last: i64) -> Option<ControlFlow<(), i64>>;
    }

    impl Equ {
        pub fn solvable(&self, operators: &[&dyn Operator]) -> bool {
            SolveStep::new(self).solvable(operators)
        }
    }

    impl std::str::FromStr for Equ {
        type Err = std::convert::Infallible;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.trim_end().split([':', ' ']);
            let result = parts.next().unwrap().parse().unwrap();
            parts.next();
            let numbers = parts.map(|n| n.parse().unwrap()).collect();

            Ok(Self { result, numbers })
        }
    }

    impl<'a> SolveStep<'a> {
        const WIN: Self = Self {
            result: 0,
            numbers: &[],
        };

        pub fn new(equ: &'a Equ) -> Self {
            Self {
                result: equ.result,
                numbers: &equ.numbers,
            }
        }

        pub fn solvable(&self, operators: &[&dyn Operator]) -> bool {
            if self.numbers.is_empty() {
                return self.result == 0;
            }

            let (last, numbers) = self.numbers.split_last().unwrap();

            operators
                .iter()
                .filter_map(|op| {
                    op.rollback(self.result, *last).map(|ctl| match ctl {
                        ControlFlow::Break(()) => Self::WIN,
                        ControlFlow::Continue(result) => Self { result, numbers },
                    })
                })
                .any(|x| x.solvable(operators))
        }
    }

    fn data() -> impl Iterator<Item = Equ> {
        read_by_line("tests/data/day7.input.txt").map(|line| line.parse().unwrap())
    }

    struct Add;
    impl Operator for Add {
        fn rollback<'a>(&self, result: i64, last: i64) -> Option<ControlFlow<(), i64>> {
            Some(ControlFlow::Continue(result - last))
        }
    }

    struct Multiple;
    impl Operator for Multiple {
        fn rollback<'a>(&self, result: i64, last: i64) -> Option<ControlFlow<(), i64>> {
            if last == 0 && result == 0 {
                Some(ControlFlow::Break(()))
            } else if result % last == 0 {
                Some(ControlFlow::Continue(result / last))
            } else {
                None
            }
        }
    }

    struct Join;
    impl Operator for Join {
        fn rollback<'a>(&self, result: i64, last: i64) -> Option<ControlFlow<(), i64>> {
            // PERF: I know I can use some math there, but to_string is just more convenient...
            let rs = result.to_string();
            let ls = last.to_string();
            if rs.ends_with(&ls) {
                rs[0..rs.len() - ls.len()]
                    .parse()
                    .ok()
                    .map(ControlFlow::Continue)
            } else {
                None
            }
        }
    }

    fn calculate(operators: &[&dyn Operator]) -> i64 {
        data()
            .filter(|equ| equ.solvable(operators))
            .map(|x| x.result)
            .sum()
    }

    #[test]
    fn part1() {
        assert_eq!(calculate(&[&Add, &Multiple]), 465126289353);
    }

    #[test]
    fn part2() {
        assert_eq!(calculate(&[&Add, &Multiple, &Join]), 70597497486371);
    }
}
