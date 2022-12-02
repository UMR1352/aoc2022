trait AsPoints {
    fn as_points(&self) -> u32;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sign {
    Rock,
    Paper,
    Scissor,
}

impl Sign {
    pub fn beats(&self, other: &Sign) -> Outcome {
        use Sign::*;
        if self == other {
            Outcome::Draw
        } else {
            match (self, other) {
                (Rock, Scissor) => Outcome::Win,
                (Scissor, Paper) => Outcome::Win,
                (Paper, Rock) => Outcome::Win,
                _ => Outcome::Lose,
            }
        }
    }
    pub fn new_from_outcome(other: &Self, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Draw => *other,
            Outcome::Win => ((*other as usize + 1) % 3).into(),
            Outcome::Lose => (*other as usize).checked_sub(1).unwrap_or(2).into(),
        }
    }
}

impl AsPoints for Sign {
    fn as_points(&self) -> u32 {
        match self {
            Sign::Rock => 1,
            Sign::Paper => 2,
            Sign::Scissor => 3,
        }
    }
}

impl From<u8> for Sign {
    fn from(c: u8) -> Self {
        match c {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissor,
            _ => unreachable!(),
        }
    }
}

impl From<usize> for Sign {
    fn from(x: usize) -> Self {
        match x {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissor,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl AsPoints for Outcome {
    fn as_points(&self) -> u32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl From<u8> for Outcome {
    fn from(c: u8) -> Self {
        match c {
            b'X' => Self::Lose,
            b'Y' => Self::Draw,
            b'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day2, part1)]
fn generator(input: &str) -> Vec<(Sign, Sign)> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            (bytes[0].into(), bytes[2].into())
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[(Sign, Sign)]) -> u32 {
    input
        .iter()
        .map(|(other, mine)| mine.as_points() + mine.beats(other).as_points())
        .sum()
}

#[aoc_generator(day2, part2)]
fn generator2(input: &str) -> Vec<(Sign, Outcome)> {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            (bytes[0].into(), bytes[2].into())
        })
        .collect()
}

#[aoc(day2, part2)]
fn part2(input: &[(Sign, Outcome)]) -> u32 {
    input
        .iter()
        .map(|(other, outcome)| {
            outcome.as_points() + Sign::new_from_outcome(other, outcome).as_points()
        })
        .sum()
}
