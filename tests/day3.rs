#[cfg(test)]
mod test {
    use advent2024::*;

    fn data() -> impl Iterator<Item = String> {
        read_by_line("tests/data/day3.input.txt")
    }

    fn parse(switch: bool) -> u32 {
        enum State {
            Init,
            M,
            U,
            L,
            LeftPara,
            Num1(String),
            Comma(u32),
            Num2(u32, String),
        }

        let mut result = 0;
        let mut enable = true;
        let mut queue = std::collections::VecDeque::with_capacity(1024);
        for line in data() {
            let mut state = State::Init;
            for c in line.chars() {
                if switch {
                    while queue.len() >= 7 {
                        queue.pop_front();
                    }
                    queue.push_back(c);
                    if c == ')' {
                        let s = queue.make_contiguous();
                        if matches!(s.last_chunk(), Some(['d', 'o', '(', ')'])) {
                            enable = true;
                        }
                        if matches!(s.last_chunk(), Some(['d', 'o', 'n', '\'', 't', '(', ')'])) {
                            enable = false;
                        }
                    }
                }
                state = match (state, c) {
                    (State::Init, 'm') => State::M,
                    (State::M, 'u') => State::U,
                    (State::U, 'l') => State::L,
                    (State::L, '(') => State::LeftPara,
                    (State::LeftPara, c @ '0'..='9') => State::Num1(String::from(c)),
                    (State::Num1(mut s), c @ '0'..='9') if s.len() < 3 => {
                        s.push(c);
                        State::Num1(s)
                    }
                    (State::Num1(s), ',') => State::Comma(s.parse().unwrap()),
                    (State::Comma(num1), c @ '0'..='9') => State::Num2(num1, String::from(c)),
                    (State::Num2(num1, mut s), c @ '0'..='9') if s.len() < 3 => {
                        s.push(c);
                        State::Num2(num1, s)
                    }
                    (State::Num2(num1, s), ')') => {
                        let num2: u32 = s.parse().unwrap();
                        if enable {
                            result += num1 * num2;
                        }
                        State::Init
                    }
                    _ => State::Init,
                }
            }
        }

        result
    }

    #[test]
    fn part1() {
        assert_eq!(parse(false), 183380722);
    }

    #[test]
    fn part2() {
        assert_eq!(parse(true), 82733683);
    }
}
