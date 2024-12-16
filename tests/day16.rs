#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet, VecDeque};

    use advent2024::*;

    #[derive(Debug, Clone, Copy)]
    enum Tile {
        Empty,
        Wall,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    impl Direction {
        fn offset(&self) -> (isize, isize) {
            match self {
                Self::Left => (0, -1),
                Self::Right => (0, 1),
                Self::Up => (-1, 0),
                Self::Down => (1, 0),
            }
        }

        fn c(&self) -> Self {
            match self {
                Self::Left => Self::Up,
                Self::Right => Self::Down,
                Self::Up => Self::Right,
                Self::Down => Self::Left,
            }
        }

        fn cc(&self) -> Self {
            match self {
                Self::Left => Self::Down,
                Self::Right => Self::Up,
                Self::Up => Self::Left,
                Self::Down => Self::Right,
            }
        }

        fn rb(&self, action: Action) -> Self {
            match action {
                Action::Go => *self,
                Action::Clockwise => self.cc(),
                Action::CounterClockwise => self.c(),
            }
        }

        fn invert(&self) -> Self {
            match self {
                Self::Left => Self::Right,
                Self::Right => Self::Left,
                Self::Up => Self::Down,
                Self::Down => Self::Up,
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum Action {
        Go,
        Clockwise,
        CounterClockwise,
    }

    struct MazePath(Vec<((usize, usize), Direction)>);

    impl MazePath {
        pub fn score(&self) -> usize {
            let mut result = 0;
            self.0.iter().map(|(_, dir)| dir).reduce(|dir, d2| {
                if dir == d2 {
                    result += 1;
                } else {
                    result += 1000;
                }
                d2
            });

            result
        }
    }

    type Score = usize;
    type Pos = (usize, usize);

    #[derive(Debug)]
    struct MazeSolver<'a> {
        maze: &'a Maze,
        record: HashMap<Pos, HashMap<Direction, (Score, HashSet<Action>)>>,
        queue: VecDeque<(Pos, Direction, Score)>,
    }

    impl<'a> MazeSolver<'a> {
        pub fn new(maze: &'a Maze) -> Self {
            let record = HashMap::new();
            let queue = VecDeque::new();
            Self {
                maze,
                record,
                queue,
            }
        }

        fn enqueue(&mut self, action: Action, pos: (usize, usize), dir: Direction, score: usize) {
            let entry = self.record.entry(pos).or_default();
            let record = entry.get_mut(&dir);
            if record.as_ref().is_none_or(|(last, _)| *last > score) {
                self.queue.push_back((pos, dir, score));
                entry.insert(dir, (score, Some(action).into_iter().collect()));
            } else if record.as_ref().is_some_and(|(last, _)| *last == score) {
                record.unwrap().1.insert(action);
            }
        }

        fn scan(&mut self) -> Option<Vec<(Direction, Score)>> {
            self.enqueue(Action::Go, self.maze.start, Direction::Right, 0);

            while let Some((pos, dir, score)) = self.queue.pop_front() {
                let new_pos = Maze::move_dir(pos, dir);
                if matches!(self.maze.get(new_pos), Tile::Empty) {
                    self.enqueue(Action::Go, new_pos, dir, score + 1);
                }
                self.enqueue(Action::Clockwise, pos, dir.c(), score + 1000);
                self.enqueue(Action::CounterClockwise, pos, dir.cc(), score + 1000);
            }

            let ends = self
                .record
                .get(&self.maze.end)?
                .iter()
                .map(|(dir, (score, _))| (*dir, *score))
                .collect();

            Some(ends)
        }

        pub fn solve(mut self) -> Option<Vec<MazePath>> {
            let ends = self.scan()?;
            let min_score = ends.iter().min_by_key(|(_, score)| score).unwrap().1;

            let mut paths = Vec::new();
            let mut path = Vec::new();

            let min_score_dirs = ends
                .into_iter()
                .filter(|(_, score)| score == &min_score)
                .map(|(dir, _)| dir);

            struct Global<'a> {
                solver: &'a MazeSolver<'a>,
                path: &'a mut Vec<(Pos, Direction)>,
                paths: &'a mut Vec<MazePath>,
            }

            struct Local {
                actions: Vec<Action>,
                index: usize,
            }

            struct Frame {
                pos: Pos,
                dir: Direction,
                locals: Option<Local>,
            }

            impl Frame {
                pub fn new(pos: Pos, dir: Direction) -> Self {
                    Self {
                        pos,
                        dir,
                        locals: None,
                    }
                }

                fn init(mut self, global: &mut Global) -> [Option<Self>; 2] {
                    if let Some((_, actions)) = global
                        .solver
                        .record
                        .get(&self.pos)
                        .and_then(|dirs| dirs.get(&self.dir))
                    {
                        global.path.push((self.pos, self.dir));
                        if self.pos == global.solver.maze.start
                            && matches!(self.dir, Direction::Right)
                        {
                            global.paths.push({
                                let mut p = global.path.clone();
                                p.reverse();
                                MazePath(p)
                            });
                        }
                        self.locals = Some(Local {
                            actions: actions.iter().copied().collect::<Vec<_>>(),
                            index: 0,
                        });
                        [Some(self), None]
                    } else {
                        [None, None]
                    }
                }

                fn logic(
                    pos: Pos,
                    dir: Direction,
                    mut local: Local,
                    global: &mut Global,
                ) -> [Option<Self>; 2] {
                    if local.index < local.actions.len() {
                        let action = local.actions[local.index];
                        local.index += 1;
                        let last_pos = if matches!(action, Action::Go) {
                            Maze::move_dir(pos, dir.invert())
                        } else {
                            pos
                        };
                        let last_dir = dir.rb(action);
                        [
                            Some(Self {
                                pos,
                                dir,
                                locals: Some(local),
                            }),
                            Some(Self::new(last_pos, last_dir)),
                        ]
                    } else {
                        global.path.pop();
                        [None, None]
                    }
                }

                pub fn execute(self, global: &mut Global) -> [Option<Self>; 2] {
                    if let Self {
                        pos,
                        dir,
                        locals: Some(local),
                    } = self
                    {
                        Self::logic(pos, dir, local, global)
                    } else {
                        self.init(global)
                    }
                }
            }

            let mut globals = Global {
                solver: &self,
                path: &mut path,
                paths: &mut paths,
            };

            for dir in min_score_dirs {
                globals.path.clear();
                let mut stack: Vec<Frame> = vec![Frame::new(self.maze.end, dir)];
                while let Some(frame) = stack.pop() {
                    frame
                        .execute(&mut globals)
                        .into_iter()
                        .flatten()
                        .for_each(|f| stack.push(f));
                }
            }

            Some(paths)
        }
    }

    #[derive(Debug)]
    struct Maze {
        map: Vec<Vec<Tile>>,
        start: (usize, usize),
        end: (usize, usize),
    }

    impl Maze {
        pub fn get(&self, (y, x): (usize, usize)) -> Tile {
            self.map[y][x]
        }

        pub fn move_dir((y, x): (usize, usize), dir: Direction) -> (usize, usize) {
            let (oy, ox) = dir.offset();
            ((y as isize + oy) as usize, (x as isize + ox) as usize)
        }
    }

    fn data(path: &str) -> Maze {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let map: Vec<_> = read_by_line(path)
            .enumerate()
            .map(|(row, line)| {
                line.into_bytes()
                    .into_iter()
                    .filter(|b| !matches!(b, b'\r' | b'\n'))
                    .enumerate()
                    .map(|(col, b)| match b {
                        b'#' => Tile::Wall,
                        b'.' => Tile::Empty,
                        b'S' => {
                            start = (row, col);
                            Tile::Empty
                        }
                        b'E' => {
                            end = (row, col);
                            Tile::Empty
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Maze { map, start, end }
    }

    #[test]
    fn part1_2() {
        let maze = data("tests/data/day16.input.txt");
        let paths = MazeSolver::new(&maze).solve().unwrap();

        assert_eq!(paths[0].score(), 102460);

        let path_tiles_count = paths
            .into_iter()
            .flat_map(|p| p.0.into_iter().map(|(pos, _)| pos))
            .collect::<HashSet<_>>()
            .len();

        assert_eq!(path_tiles_count, 527);
    }
}
