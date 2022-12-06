use itertools::Itertools;

#[aoc(day6, part1)]
fn part1(input: &[u8]) -> usize {
    input
        .windows(4)
        .enumerate()
        .find_map(|(offset, bytes)| bytes.iter().all_unique().then(|| offset + 4))
        .unwrap()
}

#[aoc(day6, part2)]
fn part2(input: &[u8]) -> usize {
    input
        .windows(14)
        .enumerate()
        .find_map(|(offset, bytes)| bytes.iter().all_unique().then(|| offset + 14))
        .unwrap()
}