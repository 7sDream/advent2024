#[cfg(test)]
mod tests {
    use std::collections::{HashMap, VecDeque};

    use advent2024::*;

    #[derive(Debug, Clone)]
    struct CPU {
        program: Vec<u8>,
        a: usize,
        b: usize,
        c: usize,
        pc: usize,
        output: Vec<u8>,
    }

    impl CPU {
        fn new(program: Vec<u8>, a: usize, b: usize, c: usize) -> Self {
            Self {
                program,
                a,
                b,
                c,
                pc: 0,
                output: Vec::new(),
            }
        }

        fn run(mut self) -> Vec<u8> {
            while self.pc + 1 < self.program.len() {
                let op: OpCode = unsafe { std::mem::transmute(self.program[self.pc]) };
                let operand = self.program[self.pc + 1];
                op.execute(&mut self, operand);
            }
            self.output
        }
    }

    trait Operand {
        fn operand(&self, cpu: &CPU, value: u8) -> usize;
    }

    struct LiteralOperand;
    impl Operand for LiteralOperand {
        fn operand(&self, _cpu: &CPU, value: u8) -> usize {
            value as usize
        }
    }

    struct ComboOperand;
    impl Operand for ComboOperand {
        fn operand(&self, cpu: &CPU, value: u8) -> usize {
            match value {
                0..=3 => value as usize,
                4 => cpu.a,
                5 => cpu.b,
                6 => cpu.c,
                _ => unreachable!(),
            }
        }
    }

    #[derive(Debug)]
    #[repr(u8)]
    #[allow(dead_code)] // because we use unsafe transmute to construct those variants
    enum OpCode {
        Adv,
        Bxl,
        Bst,
        Jnz,
        Bxc,
        Out,
        Bdv,
        Cdv,
    }

    impl OpCode {
        fn operand(&self, cpu: &CPU, value: u8) -> usize {
            match self {
                Self::Adv | Self::Bdv | Self::Cdv | Self::Bst | Self::Out => {
                    ComboOperand.operand(cpu, value)
                }
                Self::Bxl | Self::Jnz | Self::Bxc => LiteralOperand.operand(cpu, value),
            }
        }

        fn xdv(numerator: usize, operand: usize, register: &mut usize) {
            if (numerator.ilog2() as usize) < operand {
                *register = 0
            } else {
                *register = numerator >> operand
            }
        }

        pub fn execute(&self, cpu: &mut CPU, value: u8) {
            let operand = self.operand(cpu, value);
            match self {
                Self::Adv => Self::xdv(cpu.a, operand, &mut cpu.a),
                Self::Bxl => cpu.b ^= operand,
                Self::Bst => cpu.b = operand % 8,
                Self::Jnz => (),
                Self::Bxc => cpu.b ^= cpu.c,
                Self::Out => cpu.output.push((operand % 8) as u8),
                Self::Bdv => Self::xdv(cpu.a, operand, &mut cpu.b),
                Self::Cdv => Self::xdv(cpu.a, operand, &mut cpu.c),
            }
            if matches!(self, Self::Jnz) && cpu.a != 0 {
                cpu.pc = operand
            } else {
                cpu.pc += 2
            }
        }
    }

    fn data(path: &str) -> CPU {
        let mut lines = read_by_line(path);
        let mut it = lines
            .by_ref()
            .take(3)
            .map(|line| line.split(':').nth(1).unwrap().trim().parse().unwrap());

        let a = it.next().unwrap();
        let b = it.next().unwrap();
        let c = it.next().unwrap();

        lines.next(); // skip empty line

        let program = lines
            .next()
            .unwrap()
            .split([':', ','])
            .skip(1)
            .map(|x| x.trim().parse().unwrap())
            .collect();

        CPU::new(program, a, b, c)
    }

    #[test]
    fn part1() {
        let cpu = data("tests/data/day17.input.txt");
        let output: String = cpu
            .run()
            .iter()
            .flat_map(|x| [',', (b'0' + x) as char])
            .skip(1)
            .collect();

        assert_eq!(output, "2,0,1,3,4,0,2,1,7");
    }

    #[derive(Debug, Clone)]
    struct RegistryGuess(Vec<Option<bool>>);

    impl RegistryGuess {
        pub fn new(bits: usize) -> Self {
            Self(vec![None; bits])
        }

        fn to_bools(num: u8) -> [bool; 3] {
            [num & 1, num >> 1 & 1, num >> 2 & 1].map(|x| x != 0)
        }

        pub fn try_place_3(&mut self, bit: usize, num: u8) -> bool {
            let new3 = Self::to_bools(num);
            for (i, x) in new3.iter().copied().enumerate() {
                if bit + i >= self.0.len() {
                    if x {
                        return false;
                    }
                } else if self.0[bit + i].is_some_and(|c| c != x) {
                    return false;
                }
            }

            for (i, x) in new3.iter().copied().enumerate() {
                self.0.get_mut(bit + i).map(|b| b.replace(x));
            }

            true
        }

        pub fn number(&self) -> u64 {
            self.0
                .iter()
                .enumerate()
                .map(|(i, x)| 2u64.pow(i as u32) * (x.unwrap() as u64))
                .sum()
        }
    }

    /// This code only works for my input, as the solution code is somehow depends on the program's
    /// behavior, I analyzed it as:
    ///
    /// ```rust
    /// let mut A = 0;            // Some init value
    /// let mut B = 0;
    /// let mut C = 0;
    /// while A != 0 {
    ///     B = A % 8;            // B1 = last 3 bit of A
    ///     B = B ^ 3;            // B2 = B1 ^ 011
    ///     C = A >> B;           // C  = next 3 bit of A from position B2
    ///     B = B ^ C;            // B4 = B2 ^ C
    ///     B = B ^ 5;            // B5 = B4 ^ 101
    ///     print!("{}", B % 8);
    ///     A = A >> 3;
    /// }
    /// ```
    ///
    /// The output of each iteration is B5, which equals B1 ^ 011 ^ C ^ 101, while:
    ///      B1 is last 3 bit of A
    ///      C is 3 bit start from (B1 ^ 011) position of A
    ///
    /// So basic the program can be simplified as:
    ///
    /// ```txt
    /// while A > 0 {
    ///     let B = A[0..3];
    ///     let C = B ^ 011;
    ///     let H = A[C..C+3];
    ///     OUTPUT B ^ H ^ 110;
    ///     A >>= 3;
    /// }
    ///
    /// In each iteration, B ^ H ^ 110 = OUTPUT
    /// and B & H is all 3 bit number, which is 0..8.
    /// So we can calculate all possible B and H 's output, and in each output, try
    /// put B and H back in A.
    /// If we can finish all A's bit without any conflict, then we get the answer.
    /// ```
    #[test]
    fn part2() {
        let cpu = data("tests/data/day17.input.txt");

        let map = (0..8)
            .flat_map(|a| (0..8).map(move |b| ((a, b), a ^ b ^ 6)))
            .fold(HashMap::<_, Vec<_>>::new(), |mut map, (input, output)| {
                map.entry(output).or_default().push(input);
                map
            });

        let target = cpu.program;
        let mut answers = Vec::new();

        let mut q: VecDeque<(RegistryGuess, usize)> =
            Some((RegistryGuess::new(3 * target.len()), 0))
                .into_iter()
                .collect();

        while let Some((rg, i)) = q.pop_front() {
            let bit = 3 * i;

            // highest 3 bit must not be zero
            if i == target.len()
                && rg
                    .0
                    .last_chunk::<3>()
                    .unwrap()
                    .iter()
                    .any(|x| x.is_some_and(|x| x))
            {
                answers.push(rg.number());
                continue;
            }

            let output = target[i];

            if let Some(inputs) = map.get(&output) {
                for (current, high) in inputs {
                    let mut rg = rg.clone();
                    if rg.try_place_3(bit, *current)
                        && rg.try_place_3(bit + (*current as usize ^ 3), *high)
                    {
                        q.push_back((rg, i + 1));
                    }
                }
            }
        }

        answers.sort();

        assert_eq!(answers[0], 236580836040301);
    }
}
