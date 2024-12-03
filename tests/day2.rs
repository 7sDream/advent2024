#[cfg(test)]
mod test {
    use std::{cmp::Ordering, str::SplitAsciiWhitespace};

    use advent2024::*;

    fn data() -> impl Iterator<Item = impl Iterator<Item = u32>> {
        struct Report(*mut str, SplitAsciiWhitespace<'static>);

        impl Report {
            pub fn new(s: String) -> Self {
                let ptr = Box::into_raw(s.into_boxed_str());
                Self(
                    ptr,
                    unsafe { ptr.as_ref() }.unwrap().split_ascii_whitespace(),
                )
            }
        }

        impl Iterator for Report {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                self.1.next().and_then(|x| x.parse().ok())
            }
        }

        impl Drop for Report {
            fn drop(&mut self) {
                drop(unsafe { Box::from_raw(self.0) });
            }
        }

        read_by_line("tests/data/day2.input.txt").map(Report::new)
    }

    #[test]
    fn part1() {
        fn check(line: impl IntoIterator<Item = u32>) -> bool {
            let mut acc = (None, None);
            for level in line {
                acc = match acc {
                    (None, _) => (Some(level), None),
                    (Some(last), target_ord) => {
                        if level == last || level.abs_diff(last) > 3 {
                            return false;
                        }
                        let ord = level.cmp(&last);
                        if target_ord.is_some_and(|t| t != ord) {
                            return false;
                        }
                        (Some(level), Some(ord))
                    }
                }
            }

            true
        }

        let result = data().map(check).filter(|x| *x).count();

        assert_eq!(result, 282)
    }

    #[test]
    fn part2() {
        fn check(line: impl IntoIterator<Item = u32>) -> bool {
            #[derive(Clone, Default)]
            struct Task {
                index: usize,
                last: Option<u32>,
                ord: Option<Ordering>,
                skipped: bool,
            }

            let report: Vec<u32> = line.into_iter().collect();

            fn sub_check(report: &[u32], mut task: Task) -> bool {
                if task.index >= report.len() {
                    return true;
                }

                let current = report[task.index];

                let mut new_task = task.clone();
                new_task.index += 1;
                new_task.last = Some(current);
                let mut result = match task.last {
                    None => sub_check(report, new_task),
                    Some(last) => {
                        if current == last || current.abs_diff(last) > 3 {
                            false
                        } else {
                            match new_task.ord {
                                None => {
                                    new_task.ord = Some(current.cmp(&last));
                                    sub_check(report, new_task)
                                }
                                Some(ord) => {
                                    if current.cmp(&last) != ord {
                                        false
                                    } else {
                                        sub_check(report, new_task)
                                    }
                                }
                            }
                        }
                    }
                };

                if !result && !task.skipped {
                    task.skipped = true;
                    task.index += 1;
                    result = sub_check(report, task);
                }

                result
            }

            sub_check(&report, Task::default())
        }

        let result = data().map(check).filter(|x| *x).count();

        assert_eq!(result, 349)
    }
}
