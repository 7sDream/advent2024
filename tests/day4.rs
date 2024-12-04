#[cfg(test)]
mod test {
    use advent2024::*;

    fn data() -> Vec<Vec<char>> {
        read_by_line("tests/data/day4.input.txt")
            .map(|s| s.chars().filter(|c| !matches!(c, '\r' | '\n')).collect())
            .collect()
    }

    /// Get an iterator that generates all position in the input, in format of (row, col)
    fn pos(input: &[Vec<char>]) -> impl Iterator<Item = (usize, usize)> + '_ {
        input
            .iter()
            .enumerate()
            .flat_map(|(i, line)| line.iter().enumerate().map(move |(j, _)| (i, j)))
    }

    #[test]
    fn part1() {
        /// An iterator that walk from `index` position follow the `direction`
        struct Walker<'a> {
            input: &'a [Vec<char>],
            index: (isize, isize),
            direction: (isize, isize),
        }

        impl<'a> Walker<'a> {
            pub fn new(input: &'a [Vec<char>], index: (usize, usize)) -> Self {
                Self {
                    input,
                    index: (index.0 as isize, index.1 as isize),
                    direction: (0, 0),
                }
            }

            pub fn direction(mut self, direction: (isize, isize)) -> Self {
                self.direction = direction;
                self
            }

            pub fn chars(&mut self) -> [Option<char>; 4] {
                std::array::from_fn(|_| self.next())
            }
        }

        impl Iterator for Walker<'_> {
            type Item = char;

            fn next(&mut self) -> Option<Self::Item> {
                let (line, col) = self.index;
                if line < 0 || col < 0 {
                    return None;
                }

                let c = self.input.get(line as usize)?.get(col as usize)?;

                let (y, x) = self.direction;
                self.index = (line + y, col + x);

                Some(*c)
            }
        }

        fn check(chars: &[Option<char>; 4]) -> bool {
            matches!(
                chars,
                [Some('X'), Some('M'), Some('A'), Some('S')]
                    | [Some('S'), Some('A'), Some('M'), Some('X')]
            )
        }

        fn check_position(input: &[Vec<char>], position: (usize, usize)) -> usize {
            [(0, 1), (1, 0), (1, 1), (1, -1)]
                .into_iter()
                .map(|dir| Walker::new(input, position).direction(dir).chars())
                .filter(check)
                .count()
        }

        let input = data();
        let result = pos(&input)
            .map(|pos| check_position(&input, pos))
            .sum::<usize>();

        assert_eq!(result, 2358);
    }

    #[test]
    fn part2() {
        struct CrossWalker<'a> {
            input: &'a [Vec<char>],
            index: (isize, isize),
            step: usize,
        }

        impl<'a> CrossWalker<'a> {
            pub fn new(input: &'a [Vec<char>], index: (usize, usize)) -> Self {
                Self {
                    input,
                    index: (index.0 as isize, index.1 as isize),
                    step: 0,
                }
            }

            pub fn get5(&mut self) -> [Option<char>; 5] {
                std::array::from_fn(|_| self.next())
            }
        }

        impl Iterator for CrossWalker<'_> {
            type Item = char;

            fn next(&mut self) -> Option<Self::Item> {
                static STEP_OFFSET: [(isize, isize); 5] =
                    [(0, 0), (-1, -1), (1, 1), (1, -1), (-1, 1)];

                if self.step > 4 {
                    return None;
                }

                let (oy, ox) = STEP_OFFSET[self.step];
                let (mut line, mut col) = self.index;
                line += oy;
                col += ox;
                self.step += 1;

                if line < 0 || col < 0 {
                    return None;
                }

                Some(*self.input.get(line as usize)?.get(col as usize)?)
            }
        }

        fn check(chars: &[Option<char>; 5]) -> bool {
            matches!(chars[0], Some('A'))
                && matches!(
                    &chars[1..=2],
                    [Some('M'), Some('S')] | [Some('S'), Some('M')]
                )
                && matches!(
                    &chars[3..=4],
                    [Some('M'), Some('S')] | [Some('S'), Some('M')]
                )
        }

        let input = data();

        let result = pos(&input)
            .map(|position| CrossWalker::new(&input, position).get5())
            .filter(check)
            .count();

        assert_eq!(result, 1737);
    }
}
