#[cfg(test)]
mod tests {
    use std::{collections::HashSet, io::Write, path::Path};

    use advent2024::*;

    #[derive(Debug, Clone)]
    struct Robot {
        pos: (usize, usize),
        v: (isize, isize),
    }

    impl Robot {
        pub fn go(&mut self, step: usize, size: (usize, usize)) {
            let mut pos = (
                self.pos.0 as isize + self.v.0 * step as isize,
                self.pos.1 as isize + self.v.1 * step as isize,
            );
            if !(0..size.0 as isize).contains(&pos.0) {
                pos.0 = pos.0.rem_euclid(size.0 as isize);
            }
            if !(0..size.1 as isize).contains(&pos.1) {
                pos.1 = pos.1.rem_euclid(size.1 as isize);
            }
            self.pos = (pos.0 as usize, pos.1 as usize)
        }
    }

    impl std::str::FromStr for Robot {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut parts = s.trim_end().split(' ');
            let mut pos = parts.next().ok_or(())?[2..].split(',');
            let mut v = parts.next().ok_or(())?[2..].split(',');

            fn next_number<'a, Num: std::str::FromStr>(
                iter: impl IntoIterator<Item = &'a str>,
            ) -> Result<Num, ()> {
                iter.into_iter()
                    .next()
                    .ok_or(())?
                    .parse::<Num>()
                    .map_err(|_| ())
            }

            let pos = (next_number(&mut pos)?, next_number(&mut pos)?);
            let v = (next_number(&mut v)?, next_number(&mut v)?);

            Ok(Self { pos, v })
        }
    }

    fn data(path: &str) -> impl Iterator<Item = Robot> {
        read_by_line(path).filter_map(|line| line.parse().ok())
    }

    fn safe_factor(
        iter: impl IntoIterator<Item = Robot>,
        step: usize,
        size: (usize, usize),
    ) -> usize {
        let halfx = size.0 / 2;
        let halfy = size.1 / 2;

        iter.into_iter()
            .map(|mut robot| {
                robot.go(step, size);
                robot.pos
            })
            .fold([0; 4], |mut quadrant, (x, y)| {
                if x != halfx && y != halfy {
                    let qx = if x < halfx { 0 } else { 1 } + if y < halfy { 0 } else { 2 };
                    quadrant[qx] += 1;
                }
                quadrant
            })
            .into_iter()
            .product()
    }

    #[test]
    fn part1() {
        assert_eq!(
            safe_factor(data("tests/data/day14.input.txt"), 100, (101, 103)),
            219512160,
        );
    }

    // This will create 10000 ppm file in your tests/data/day14 folder.
    // So I comment the test attribute bellow, so it will not run by default

    // #[test]
    fn part2() {
        let robots: Vec<_> = data("tests/data/day14.input.txt").collect();

        let dir = Path::new("tests/data/day14");
        std::fs::create_dir_all(dir).unwrap();

        fn write_ppm(
            dir: impl AsRef<Path>,
            filename: impl std::fmt::Display,
            (height, width): (usize, usize),
            points: impl IntoIterator<Item = (usize, usize)>,
        ) {
            let f: std::fs::File = std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(dir.as_ref().join(format!("{}.ppm", filename)))
                .unwrap();
            let mut writer = std::io::BufWriter::new(f);

            writer.write_all(b"P3\n").unwrap();
            writer
                .write_fmt(format_args!("{} {}\n1\n", width, height))
                .unwrap();

            let mut cy = 0;
            let mut cx = 0;
            for (y, x) in points.into_iter().chain(Some((height, 0))) {
                while cy != y {
                    writer
                        .write_all(" 0 0 0".repeat(width - cx).as_bytes())
                        .unwrap();
                    writer.write_all(b"\n").unwrap();
                    cy += 1;
                    cx = 0;
                }
                if y >= height {
                    break;
                }
                if cx != x {
                    writer
                        .write_all(" 0 0 0".repeat(x - cx).as_bytes())
                        .unwrap();
                }
                writer.write_all(b" 1 1 1").unwrap();
                cx = x + 1
            }
        }

        (1..10000).for_each(|step| {
            let points: HashSet<_> = robots
                .iter()
                .cloned()
                .map(|mut x| {
                    x.go(step, (101, 103));
                    x.pos
                })
                .map(|(x, y)| (y, x))
                .collect();
            let mut points: Vec<_> = points.into_iter().collect();
            points.sort();
            write_ppm(dir, step, (103, 101), points);
        });
    }
}
