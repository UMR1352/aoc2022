use itertools::{repeat_n, Itertools};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
enum Opcode {
    #[default]
    Noop,
    AddX,
}

type Cycles = u8;

struct Instruction {
    _opcode: Opcode,
    cycles: Cycles,
    arg: Option<i64>,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            _opcode: Opcode::Noop,
            cycles: 1,
            arg: None,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split_once(' ')
            .map_or_else(Instruction::default, |(_, arg)| {
                Instruction::from_opcode(Opcode::AddX, arg.parse::<i64>().ok())
            }))
    }
}

impl Instruction {
    pub fn from_opcode(opcode: Opcode, arg: Option<i64>) -> Self {
        let cycles = match opcode {
            Opcode::Noop => 1,
            Opcode::AddX => 2,
        };
        Self {
            cycles,
            _opcode: opcode,
            arg,
        }
    }
}

#[aoc_generator(day10)]
fn generator(input: &str) -> Vec<Instruction> {
    input.lines().flat_map(Instruction::from_str).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[Instruction]) -> i64 {
    input
        .iter()
        .scan(1, |x, instruction| {
            let x_before_update = *x;
            if let Some(arg) = instruction.arg {
                *x += arg;
            }
            Some(repeat_n(x_before_update, instruction.cycles as usize))
        })
        .flatten()
        .zip(1_u64..)
        .filter_map(|(x, clock)| {
            [20, 60, 100, 140, 180, 220]
                .contains(&clock)
                .then_some(x * clock as i64)
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &[Instruction]) -> Option<i32> {
    input
        .iter()
        .scan(1, |x, instruction| {
            let x_before_update = *x;
            if let Some(arg) = instruction.arg {
                *x += arg;
            }
            Some(repeat_n(x_before_update, instruction.cycles as usize))
        })
        .flatten()
        .zip(0_i64..)
        .map(|(center_pos, clock)| {
            let scan_line = clock % 40;
            if [center_pos - 1, center_pos, center_pos + 1].contains(&scan_line) {'#'}  else {'.'}
        })
        .chunks(40)
        .into_iter()
        .for_each(|chunk| {
            chunk.for_each(|pixel| print!("{}", pixel));
            println!();
        });

    None
}