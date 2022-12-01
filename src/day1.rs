use core::cmp::Reverse;
use std::collections::BinaryHeap;

struct HighestN<T> {
    highest_n: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> HighestN<T> {
    pub fn new<I, const N: usize>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        let mut highest_n = BinaryHeap::with_capacity(N);
        for x in iter {
            highest_n.push(Reverse(x));
            if highest_n.len() > N {
                let _ = highest_n.pop();
            }
        }
        Self { highest_n }
    }
}

impl<T: Ord> Iterator for HighestN<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.highest_n.pop().map(|x| x.0)
    }
}

trait HighestNIter<T>
where
    T: Ord,
    Self: Iterator<Item = T> + Sized,
{
    fn n_highest<const N: usize>(self) -> HighestN<T> {
        HighestN::new::<Self, N>(self)
    }
}

impl<I, T> HighestNIter<T> for I
where
    I: Iterator<Item = T>,
    T: Ord,
{
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
    input.iter().n_highest::<3>().sum()
}
