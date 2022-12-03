use crate::utils::CollectInto;
use itertools::Itertools;
use std::collections::HashSet;

trait AsPriority {
    fn as_priority(&self) -> u32;
}

impl AsPriority for char {
    fn as_priority(&self) -> u32 {
        match self {
            'a'..='z' => *self as u32 - 'a' as u32 + 1,
            'A'..='Z' => *self as u32 - 'A' as u32 + 27,
            _ => unreachable!(),
        }
    }
}

#[aoc(day3, part1)]
#[allow(unstable_name_collisions)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .scan(HashSet::new(), |set, (left, right)| {
            set.clear();
            left.chars().collect_into(set);
            right
                .chars()
                .find(|c| set.contains(c))
                .map(|c| c.as_priority())
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.chars())
        .chunks(3)
        .into_iter()
        .flat_map(|chunk| {
            chunk
                .map(|items| items.collect::<HashSet<char>>())
                .reduce(|intersection, other| &intersection & &other)
                .and_then(|set| set.into_iter().next().map(|c| c.as_priority()))
        })
        .sum()
}
