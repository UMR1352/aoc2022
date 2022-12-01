use itertools::Itertools;

#[aoc_generator(day1)]
fn gerator(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|s| s.lines().flat_map(str::parse::<u32>).sum())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    *input
        .iter()
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    input
        .iter()
        .sorted()
        .rev()
        .take(3)
        .sum()
}