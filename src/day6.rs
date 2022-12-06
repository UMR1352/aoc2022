trait AllUniqueIter: Iterator {
    fn all_unique(self) -> bool;
}

impl<'a, I: Iterator<Item = &'a u8>> AllUniqueIter for I {
    fn all_unique(mut self) -> bool {
        let mut set = [0_u64; 4];
        while let Some(byte) = self.next() {
            let slot = (byte / 64) as usize;
            let offset = (byte % 64) as usize;
            if set[slot] & (1 << offset) != 0 {
                return false;
            } else {
                set[slot] |= 1 << offset;
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
