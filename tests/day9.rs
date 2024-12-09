#[cfg(test)]
mod tests {
    use advent2024::*;

    type FileID = usize;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Block {
        File(FileID),
        Empty,
    }

    type Fragment = (Block, u8);

    #[derive(Debug)]
    struct DiskMap(Vec<u8>);

    impl DiskMap {
        fn get(&self, index: usize) -> Option<Fragment> {
            let block = if index % 2 == 0 {
                Block::File(index / 2)
            } else {
                Block::Empty
            };
            Some((block, self.0.get(index).copied()?))
        }

        fn fragments(&self) -> impl Iterator<Item = Fragment> + '_ {
            (0..self.0.len()).map(|x| self.get(x).unwrap())
        }
    }

    #[derive(Debug)]
    struct Pointer<'a> {
        disk: &'a DiskMap,
        index: usize,
        fragment: Fragment,
        offset: u8,
    }

    impl<'a> Pointer<'a> {
        pub fn new(disk: &'a DiskMap, mut index: usize, rtl: bool) -> Option<Self> {
            loop {
                let fragment = disk.get(index)?;

                if fragment.1 /* count */ == 0 {
                    if rtl {
                        index = index.checked_sub(1)?;
                    } else {
                        index = index.checked_add(1)?;
                    }
                    continue;
                }

                return Some(Self {
                    disk,
                    index,
                    fragment,
                    offset: if rtl { fragment.1 - 1 } else { 0 },
                });
            }
        }

        pub fn move_right(&mut self) -> Option<&mut Self> {
            if self.offset + 1 == self.fragment.1 {
                *self = Self::new(self.disk, self.index + 1, false)?;
            } else {
                self.offset += 1;
            }
            Some(self)
        }

        pub fn move_left(&mut self) -> Option<&mut Self> {
            if self.offset == 0 {
                *self = Self::new(self.disk, self.index.checked_sub(1)?, true)?;
            } else {
                self.offset -= 1;
            }
            Some(self)
        }

        fn step(action: PointerAction, left: &mut Pointer, right: &mut Pointer) -> bool {
            match action {
                PointerAction::MoveLeftPointer => left.move_right().is_some(),
                PointerAction::MoveRightPointer => right.move_left().is_some(),
                PointerAction::MoveBoth => {
                    left.move_right().is_some() && right.move_left().is_some()
                }
                PointerAction::Noop => true,
            }
        }

        fn ending(left: &Pointer, right: &Pointer) -> bool {
            left.index > right.index || (left.index == right.index && left.offset > right.offset)
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum PointerAction {
        MoveLeftPointer,
        MoveRightPointer,
        MoveBoth,
        Noop,
    }

    impl DiskMap {
        pub fn defrag(&self) -> impl Iterator<Item = Block> + '_ {
            let mut left = Pointer::new(self, 0, false).unwrap();
            let mut right = Pointer::new(self, self.0.len() - 1, true).unwrap();
            let mut action = PointerAction::Noop;
            std::iter::from_fn(move || loop {
                if !Pointer::step(action, &mut left, &mut right) {
                    return None;
                }

                let (next_action, result) = self.defrag_impl(&mut left, &mut right);
                action = next_action;

                if let Some(result) = result {
                    return result;
                }
            })
        }

        fn defrag_impl(
            &self,
            left: &mut Pointer,
            right: &mut Pointer,
        ) -> (PointerAction, Option<Option<Block>>) {
            if Pointer::ending(left, right) {
                return (PointerAction::Noop, Some(None));
            }

            match (left.fragment.0, right.fragment.0) {
                (file @ Block::File(_), _) => (PointerAction::MoveLeftPointer, Some(Some(file))),
                (Block::Empty, file @ Block::File(_)) => {
                    (PointerAction::MoveBoth, Some(Some(file)))
                }
                (Block::Empty, Block::Empty) => (PointerAction::MoveRightPointer, None),
            }
        }
    }

    #[derive(Debug)]
    struct ContiguousDiskMap(Vec<Fragment>);

    impl FromIterator<Block> for ContiguousDiskMap {
        fn from_iter<T: IntoIterator<Item = Block>>(iter: T) -> Self {
            let disk = iter.into_iter().fold(Vec::new(), |mut disk, file| {
                if let Some((last, count)) = disk.last_mut() {
                    if file == *last {
                        *count += 1;
                        return disk;
                    }
                }
                disk.push((file, 1));
                disk
            });
            Self(disk)
        }
    }

    impl FromIterator<Fragment> for ContiguousDiskMap {
        fn from_iter<T: IntoIterator<Item = Fragment>>(iter: T) -> Self {
            Self(iter.into_iter().collect())
        }
    }

    impl ContiguousDiskMap {
        fn defrag_whole_file(&mut self) {
            let disk = &mut self.0;
            let mut idx = disk.len();
            while let Some(current) = idx.checked_sub(1) {
                let (block, count) = disk[current];
                if let Block::File(file) = block {
                    let free = disk.iter().take(idx).copied().enumerate().find(
                        |(_, (block, free_count))| {
                            matches!(block, Block::Empty) && *free_count >= count
                        },
                    );
                    if let Some((free_idx, (_, free_count))) = free {
                        disk[free_idx] = (Block::File(file), count);
                        disk[current].0 = Block::Empty;
                        if free_count > count {
                            disk.insert(free_idx + 1, (Block::Empty, free_count - count));
                        }
                    }
                }
                idx = current;
            }
        }

        fn checksum(&self) -> usize {
            let mut i = 0;
            self.0
                .iter()
                .map(|(file, count)| {
                    let sum = (i..i + *count as usize).sum::<usize>()
                        * match file {
                            Block::Empty => 0,
                            Block::File(file) => *file,
                        };
                    i += *count as usize;
                    sum
                })
                .sum()
        }
    }

    fn data() -> DiskMap {
        DiskMap(
            read_by_byte("tests/data/day9.input.txt")
                .filter(|b| b.is_ascii_digit())
                .map(|x| x - b'0')
                .collect(),
        )
    }

    #[test]
    fn part1() {
        let disk: ContiguousDiskMap = data().defrag().collect();
        assert_eq!(disk.checksum(), 6225730762521);
    }

    #[test]
    fn part2() {
        let mut disk: ContiguousDiskMap = data().fragments().collect();
        disk.defrag_whole_file();
        assert_eq!(disk.checksum(), 6250605700557);
    }
}
