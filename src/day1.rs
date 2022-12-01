use itertools::Itertools;

#[aoc_generator(day1)]
fn gerator(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|s| s.lines().flat_map(str::parse).collect())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|cals| cals.into_iter().sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|cals| cals.into_iter().sum::<u32>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}