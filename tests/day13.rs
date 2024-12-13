#[cfg(test)]
mod tests {
    use advent2024::*;

    #[derive(Debug)]
    struct ClawMachine {
        tx: i64,
        ty: i64,
        ax: i64,
        ay: i64,
        bx: i64,
        by: i64,
    }

    impl ClawMachine {
        // Maybe the input data do not have any X=0 or Y=0, so I'm not sure if I
        // handles those case properly, but I did my best...
        fn solve(&self) -> Option<(i64, i64)> {
            // If target is zero, answer is always zero
            if self.tx == 0 && self.ty == 0 {
                return Some((0, 0));
            }

            let ab = self.ax * self.by - self.ay * self.bx;
            let at = self.ax * self.ty - self.ay * self.tx;
            let bt = self.by * self.tx - self.bx * self.ty;

            // A B is same direction
            if ab == 0 {
                // but target is not in this direction
                if at != 0 || bt != 0 {
                    return None;
                }

                // both A B is zero
                if self.ax == 0 && self.ay == 0 && self.bx == 0 && self.by == 0 {
                    return None;
                }

                // if A is zero
                if self.ax == 0 && self.ay == 0 {
                    if self.bx != 0 && self.tx % self.bx == 0 {
                        return Some((0, self.tx / self.bx));
                    } else if self.by != 0 && self.ty % self.by == 0 {
                        return Some((0, self.ty / self.by));
                    } else {
                        return None;
                    }
                }

                // if B is zero
                if self.bx == 0 && self.by == 0 {
                    if self.ax != 0 && self.tx % self.ax == 0 {
                        return Some((0, self.tx / self.ax));
                    } else if self.ay != 0 && self.ty % self.ay == 0 {
                        return Some((0, self.ty / self.ay));
                    } else {
                        return None;
                    }
                }

                // No one is zero

                // b cheap then a
                if 9 * (self.bx * self.bx + self.by * self.by)
                    > (self.ax * self.ax + self.ay * self.ay)
                {
                    if let Some(a) = (0..)
                        .take_while(|a| self.tx - a * self.ax > 0 && self.ty - a * self.ay > 0)
                        .find(|a| {
                            (self.bx == 0 || (self.tx - a * self.ax) % self.bx == 0)
                                && (self.by == 0 || (self.ty - a * self.ay) % self.by == 0)
                        })
                    {
                        return Some((
                            a,
                            if self.bx == 0 {
                                (self.tx - a * self.ay) / self.by
                            } else {
                                (self.ty - a * self.ax) / self.bx
                            },
                        ));
                    }
                } else {
                    // a cheap then b
                    if let Some(b) = (0..)
                        .take_while(|b| self.tx - b * self.bx > 0 && self.ty - b * self.by > 0)
                        .find(|b| {
                            (self.ax == 0 || (self.tx - b * self.bx) % self.ax == 0)
                                && (self.ay == 0 || (self.ty - b * self.by) % self.ay == 0)
                        })
                    {
                        return Some((
                            if self.ax == 0 {
                                (self.ty - b * self.by) / self.ay
                            } else {
                                (self.tx - b * self.bx) / self.ax
                            },
                            b,
                        ));
                    }
                }

                None
            } else {
                let b = at / ab;
                let a = bt / ab;
                if a >= 0
                    && b >= 0
                    && self.ax * a + self.bx * b == self.tx
                    && self.ay * a + self.by * b == self.ty
                {
                    Some((a, b))
                } else {
                    None
                }
            }
        }
    }

    fn data(extra: i64) -> impl Iterator<Item = ClawMachine> {
        let mut iter = read_by_line("tests/data/day13.input.txt");
        std::iter::from_fn(move || {
            fn xy(line: String) -> (i64, i64) {
                let mut it = line.trim().split([':', ' ', ',']);
                let x = it
                    .find(|part| part.starts_with("X+") || part.starts_with("X="))
                    .unwrap();
                let y = it
                    .find(|part| part.starts_with("Y+") || part.starts_with("Y="))
                    .unwrap();

                (x[2..].parse().unwrap(), y[2..].parse().unwrap())
            }

            let (ax, ay) = xy(iter.next()?);
            let (bx, by) = xy(iter.next()?);
            let (tx, ty) = xy(iter.next()?);

            let _ = iter.next(); // skip empty line

            Some(ClawMachine {
                ax,
                ay,
                bx,
                by,
                tx: tx + extra,
                ty: ty + extra,
            })
        })
    }

    #[test]
    fn part1() {
        let result: i64 = data(0)
            .filter_map(|m| m.solve())
            .filter(|(a, b)| (0..=100).contains(a) && (0..=100).contains(b))
            .map(|(a, b)| 3 * a + b)
            .sum();

        assert_eq!(result, 36250);
    }

    #[test]
    fn part2() {
        let result: i64 = data(10000000000000)
            .filter_map(|m| m.solve())
            .map(|(a, b)| 3 * a + b)
            .sum();

        assert_eq!(result, 83232379451012);
    }
}
