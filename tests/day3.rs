#[cfg(test)]
mod test {
    use advent2024::*;

    fn data() -> Vec<u8> {
        read_all("tests/data/day3.input.txt")
    }

    #[derive(Debug, Clone)]
    enum Command {
        Mul(u32, u32),
        Do,
        Dont,
    }

    fn parse(s: &[u8]) -> impl Iterator<Item = Command> + '_ {
        use winnow::{ascii::*, combinator::*, token::*, PResult, Parser};

        fn mul(input: &mut &[u8]) -> PResult<Command> {
            delimited(b"mul(", separated_pair(dec_uint, b",", dec_uint), b")")
                .map(|(a, b)| Command::Mul(a, b))
                .parse_next(input)
        }

        fn command(input: &mut &[u8]) -> PResult<Option<Command>> {
            alt((
                b"do()".value(Some(Command::Do)),
                b"don't()".value(Some(Command::Dont)),
                mul.map(Some),
                any.value(None),
            ))
            .parse_next(input)
        }

        let mut it = iterator(s, command);
        std::iter::from_fn(move || (&mut it).next()).flatten()
    }

    #[test]
    fn part1() {
        let input = data();
        let result: u32 = parse(&input)
            .map(|cmd| match cmd {
                Command::Mul(a, b) => a * b,
                _ => 0,
            })
            .sum();

        assert_eq!(result, 183380722);
    }

    #[test]
    fn part2() {
        let input = data();
        let result = parse(&input)
            .fold((0u32, true), |(acc, enable), cmd| match (enable, cmd) {
                (true, Command::Mul(a, b)) => (acc + a * b, enable),
                (false, Command::Mul(_, _)) => (acc, enable),
                (_, Command::Do) => (acc, true),
                (_, Command::Dont) => (acc, false),
            })
            .0;

        assert_eq!(result, 82733683);
    }
}
