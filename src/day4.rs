use std::ops::RangeInclusive;

trait RangeOverlap {
    fn overlaps(&self, other: &Self) -> bool;
    fn fully_overlaps(&self, other: &Self) -> bool;
}

impl RangeOverlap for RangeInclusive<u32> {
    fn fully_overlaps(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || other.contains(self.start())
    }
}

fn parse_range(s: &str) -> RangeInclusive<u32> {
    let (left, right) = s.split_once('-').unwrap();
    let left = left.parse::<u32>().unwrap();
    let right = right.parse::<u32>().unwrap();

    left..=right
}

#[aoc_generator(day4)]
fn generator(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            (parse_range(left), parse_range(right))
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .iter()
        .filter(|(left, right)| left.fully_overlaps(right) || right.fully_overlaps(left))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .iter()
        .filter(|(left, right)| left.overlaps(right))
        .count()
}