trait AllUniqueIter: Iterator {
    fn all_unique(self) -> bool;
}

impl<'a, I: Iterator<Item = &'a u8>> AllUniqueIter for I {
    fn all_unique(self) -> bool {
        let mut set = 0_u32;
        for byte in self {
            let offset = (byte - b'a') as usize;
            if set & (1 << offset) != 0 {
                return false;
            } else {
                set |= 1 << offset;
            }
        }

        true
    }
}

#[aoc(day6, part1)]
fn part1(input: &[u8]) -> usize {
    input
        .windows(4)
        .enumerate()
        .find_map(|(offset, bytes)| bytes.iter().all_unique().then_some(offset + 4))
        .unwrap()
}

#[aoc(day6, part2)]
fn part2(input: &[u8]) -> usize {
    input
        .windows(14)
        .enumerate()
        .find_map(|(offset, bytes)| bytes.iter().all_unique().then_some(offset + 14))
        .unwrap()
}
