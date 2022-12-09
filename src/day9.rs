use itertools::Itertools;
use std::{ops::AddAssign, str::FromStr};

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn from_tuple(pos: (i32, i32)) -> Self {
        Self { x: pos.0, y: pos.1 }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => unreachable!(),
        }
    }
}

impl Dir {
    fn to_pos(self) -> Pos {
        match self {
            Self::Up => Pos::from_tuple((0, -1)),
            Self::Down => Pos::from_tuple((0, 1)),
            Self::Left => Pos::from_tuple((-1, 0)),
            Self::Right => Pos::from_tuple((1, 0)),
        }
    }
}

trait Movement {
    fn king_distance(&self, other: &Self) -> usize;
    fn normal_diff(&self, other: &Self) -> Self;
}

impl Movement for Pos {
    fn king_distance(&self, other: &Self) -> usize {
        let dx = self.x.abs_diff(other.x) as usize;
        let dy = self.y.abs_diff(other.y) as usize;

        dx.max(dy)
    }
    fn normal_diff(&self, other: &Self) -> Pos {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        Pos::from_tuple((dx.signum(), dy.signum()))
    }
}

#[aoc_generator(day9)]
fn generator(input: &str) -> Vec<(Dir, usize)> {
    input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(left, right)| {
                    (
                        Dir::from_str(left).unwrap(),
                        right.parse::<usize>().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect()
}

#[repr(transparent)]
#[derive(Debug)]
struct Rope<const N: usize>([Pos; N]);

impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self([Pos::default(); N])
    }
}

impl<const N: usize> Rope<N> {
    #[inline(always)]
    pub fn head_mut(&mut self) -> &mut Pos {
        &mut self.0[0]
    }
    #[inline(always)]
    pub fn head(&self) -> &Pos {
        &self.0[0]
    }
    #[inline(always)]
    pub fn tail(&self) -> &Pos {
        &self.0[N - 1]
    }
    pub fn m0ve(&mut self, dir: &Pos) {
        *self.head_mut() += *dir;
        let mut prev_knot = *self.head();
        for curr_knot in self.0[1..N].iter_mut() {
            if prev_knot.king_distance(curr_knot) > 1 {
                let diff = curr_knot.normal_diff(&prev_knot);
                *curr_knot += diff;
                prev_knot = *curr_knot;
            } else {
                break;
            }
        }
    }
}

#[aoc(day9, part1)]
fn part1(input: &[(Dir, usize)]) -> usize {
    input
        .iter()
        .flat_map(|(dir, steps)| std::iter::repeat(dir.to_pos()).take(*steps))
        .scan(Rope::<2>::default(), |rope, dir| {
            rope.m0ve(&dir);
            Some(*rope.tail())
        })
        .unique()
        .count()
}

#[aoc(day9, part2)]
fn part2(input: &[(Dir, usize)]) -> usize {
    input
        .iter()
        .flat_map(|(dir, steps)| std::iter::repeat(dir.to_pos()).take(*steps))
        .scan(Rope::<10>::default(), |rope, dir| {
            rope.m0ve(&dir);
            Some(*rope.tail())
        })
        .unique()
        .count()
}
