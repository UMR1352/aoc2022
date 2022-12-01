use core::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Debug;

trait HighestN: Iterator {
    fn highest_n<const N: usize>(self) -> [Self::Item; N];
}

impl<I: Iterator> HighestN for I
where
    I::Item: Ord + Debug,
{
    fn highest_n<const N: usize>(self) -> [Self::Item; N] {
        let queue = BinaryHeap::with_capacity(N);
        self.fold(queue, |mut queue, x| {
            queue.push(Reverse(x));
            if queue.len() > N {
                let _ = queue.pop();
            }
            queue
        })
        .into_iter()
        .map(|x| x.0)
        .collect::<Vec<Self::Item>>()
        .try_into()
        .unwrap()
    }
}

#[aoc_generator(day1)]
fn gerator(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|s| s.lines().flat_map(str::parse::<u32>).sum())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    *input.iter().max().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> u32 {
    input.iter().highest_n::<3>().into_iter().sum()
}
