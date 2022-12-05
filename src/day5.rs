use std::str::FromStr;

const EMPTY_VEC: Vec<char> = Vec::new();

#[derive(Debug)]
struct Instruction {
    amount: usize,
    source: usize,
    dest: usize,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace().flat_map(str::parse::<usize>);
        let amount = it.next().unwrap();
        let source = it.next().unwrap() - 1;
        let dest = it.next().unwrap() - 1;

        Ok(Instruction {
            amount,
            source,
            dest,
        })
    }
}

#[aoc_generator(day5)]
fn generator(input: &str) -> ([Vec<char>; 9], Vec<Instruction>) {
    let (crates_stack, instructions) = input.split_once("\n\n").unwrap();

    let crates_stack = crates_stack
        .lines()
        .rev()
        .filter(|line| line.contains("["))
        .flat_map(|line| {
            line.as_bytes()
                .chunks(4)
                .enumerate()
                .filter_map(|(i, chunk)| {
                    chunk[1]
                        .is_ascii_alphabetic()
                        .then(|| (i, chunk[1] as char))
                })
        })
        .fold([EMPTY_VEC; 9], |mut stacks, (i, crate_id)| {
            stacks[i].push(crate_id);
            stacks
        });
    let instructions = instructions
        .lines()
        .flat_map(Instruction::from_str)
        .collect();

    (crates_stack, instructions)
}

#[aoc(day5, part1)]
fn part1((stacks, instructions): &([Vec<char>; 9], Vec<Instruction>)) -> String {
    let mut stacks = stacks.clone();
    for Instruction {
        amount,
        source,
        dest,
    } in instructions.into_iter()
    {
        for _ in 0..*amount {
            stacks[*source].pop().map(|c| stacks[*dest].push(c));
        }
    }

    stacks
        .into_iter()
        .flat_map(|mut stack| stack.pop())
        .collect()
}

#[aoc(day5, part2)]
fn part2((stacks, instructions): &([Vec<char>; 9], Vec<Instruction>)) -> String {
    let mut stacks = stacks.clone();
    for Instruction {
        amount,
        source,
        dest,
    } in instructions.into_iter()
    {
        let mut temp = vec![];
        for _ in 0..*amount {
            stacks[*source].pop().map(|c| temp.push(c));
        }
        temp.into_iter().rev().for_each(|c| stacks[*dest].push(c));
    }

    stacks
        .into_iter()
        .flat_map(|mut stack| stack.pop())
        .collect()
}
