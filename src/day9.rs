use std::str::FromStr;

use itertools::Itertools;

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
    fn to_pos(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

trait Movement {
    fn king_distance(&self, other: &Self) -> usize;
    fn normal_diff(&self, other: &Self) -> (i32, i32);
}

impl Movement for (i32, i32) {
    fn king_distance(&self, other: &Self) -> usize {
        let dx = self.0.abs_diff(other.0) as usize;
        let dy = self.1.abs_diff(other.1) as usize;

        dx.max(dy)
    }
    fn normal_diff(&self, other: &Self) -> (i32, i32) {
        let dx = other.0 - self.0;
        let dy = other.1 - self.1;

        (dx.signum(), dy.signum())
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
struct Rope<const N: usize>([(i32, i32); N]);

impl<const N: usize> Default for Rope<N> {
    fn default() -> Self {
        Self([(0, 0); N])
    }
}

impl<const N: usize> Rope<N> {
    pub fn head_mut(&mut self) -> &mut (i32, i32) {
        &mut self.0[0]
    }
    pub fn head(&self) -> &(i32, i32) {
        &self.0[0]
    }
    pub fn tail(&self) -> &(i32, i32) {
        &self.0[N - 1]
    }
    #[inline(always)]
    fn move_head(&mut self, dir: &(i32, i32)) -> (i32, i32) {
        let prev @ (x, y) = *self.head();
        *self.head_mut() = (x + dir.0, y + dir.1);

        prev
    }
    pub fn m0ve(&mut self, dir: &(i32, i32)) {
        let mut _prev = self.move_head(dir);
        let mut prev_knot = *self.head();

        for curr_knot in self.0[1..N].iter_mut() {
            if prev_knot.king_distance(curr_knot) > 1 {
                let diff = curr_knot.normal_diff(&prev_knot);
                curr_knot.0 += diff.0;
                curr_knot.1 += diff.1;

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