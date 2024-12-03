#[cfg(test)]
mod test {
    use advent2024::*;

    fn data() -> impl Iterator<Item = u8> {
        read_by_byte("tests/data/day3.input.txt")
    }

    fn parse(switch: bool) -> u32 {
        enum State {
            Init,
            M,
            U,
            L,
            LeftParen,
            Num1(String),
            Comma(u32),
            Num2(u32, String),
        }

        let mut result = 0;
        let mut enable = true;
        let mut queue = std::collections::VecDeque::with_capacity(8);
        let mut state = State::Init;
        for c in data() {
            if switch {
                while queue.len() >= 7 {
                    queue.pop_front();
                }
                queue.push_back(c);
                if c == b')' {
                    let s = queue.make_contiguous();
                    if matches!(s.last_chunk(), Some([b'd', b'o', b'(', b')'])) {
                        enable = true;
                    }
                    if matches!(
                        s.last_chunk(),
                        Some([b'd', b'o', b'n', b'\'', b't', b'(', b')'])
                    ) {
                        enable = false;
                    }
                }
            }
            state = match (state, c) {
                (_, b'm') => State::M,
                (State::M, b'u') => State::U,
                (State::U, b'l') => State::L,
                (State::L, b'(') => State::LeftParen,
                (State::LeftParen, b'0'..=b'9') => State::Num1(String::from(c as char)),
                (State::Num1(mut s), b'0'..=b'9') if s.len() < 3 => {
                    s.push(c as char);
                    State::Num1(s)
                }
                (State::Num1(s), b',') => State::Comma(s.parse().unwrap()),
                (State::Comma(num1), b'0'..=b'9') => State::Num2(num1, String::from(c as char)),
                (State::Num2(num1, mut s), b'0'..=b'9') if s.len() < 3 => {
                    s.push(c as char);
                    State::Num2(num1, s)
                }
                (State::Num2(num1, s), b')') => {
                    let num2: u32 = s.parse().unwrap();
                    if enable {
                        result += num1 * num2;
                    }
                    State::Init
                }
                _ => State::Init,
            };
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
