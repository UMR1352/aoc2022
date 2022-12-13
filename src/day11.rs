use crate::utils::HighestNIter;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace1, u64 as parse_u64},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::collections::VecDeque;

#[derive(Debug, Default)]
enum Operation {
    Multiply(u64),
    #[default]
    Square,
    Sum(u64),
}

#[derive(Debug)]
struct Monkey {
    starting_items: VecDeque<u64>,
    operation: Operation,
    test_divisor: u64,
    test_branch: (usize, usize),
    thrown_items_counter: usize,
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            starting_items: VecDeque::new(),
            operation: Operation::Square,
            test_divisor: 0,
            test_branch: (0, 0),
            thrown_items_counter: 0,
        }
    }
}

impl Monkey {
    pub fn throw(&mut self, others: &mut [Monkey], worry_level_decreases: bool, test_divider: u64) {
        let divider = if worry_level_decreases { 3 } else { 1 };
        self.starting_items.drain(..).map(|level| match self.operation {
            Operation::Multiply(x) => level * x,
            Operation::Square => level * level,
            Operation::Sum(x) => level + x,
        } / divider)
        .for_each(|mut level| {
            level %= test_divider;
            if level % self.test_divisor == 0 {
                others[self.test_branch.0].catch(level);
            } else {
                others[self.test_branch.1].catch(level);
            }
            self.thrown_items_counter += 1;
        })
    }
    pub fn catch(&mut self, item: u64) {
        self.starting_items.push_back(item);
    }
}
/// Monkey 0:
///   Starting items: 89, 95, 92, 64, 87, 68
///   Operation: new = old * 11
///   Test: divisible by 2
///     If true: throw to monkey 7
///     If false: throw to monkey 4
fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (starting_items, operation, (divisor, if_true_idx, if_false_idx)) = preceded(
        delimited(tag("Monkey "), parse_u64, char(':')),
        tuple((parse_starting_items, parse_operation, parse_test)),
    )(input)?
    .1;

    Ok((
        "",
        Monkey {
            starting_items,
            operation,
            test_divisor: divisor,
            test_branch: (if_true_idx as usize, if_false_idx as usize),
            thrown_items_counter: 0,
        },
    ))
}

fn parse_starting_items(input: &str) -> IResult<&str, VecDeque<u64>> {
    let prefix = tag("Starting items: ");
    let list = separated_list1(tag(", "), parse_u64);
    map(preceded(prefix, list), VecDeque::from)(input.trim())
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    preceded(
        tag("Operation: new = old "),
        alt((
            parse_operation_sum,
            parse_operation_multiply,
            parse_operation_square,
        )),
    )(input.trim())
}

fn parse_operation_square(input: &str) -> IResult<&str, Operation> {
    tag("* old")(input).map(|(rem, _)| (rem, Operation::Square))
}

fn parse_operation_sum(input: &str) -> IResult<&str, Operation> {
    preceded(tag("+ "), parse_u64)(input).map(|(rem, value)| (rem, Operation::Sum(value)))
}

fn parse_operation_multiply(input: &str) -> IResult<&str, Operation> {
    preceded(tag("* "), parse_u64)(input).map(|(rem, value)| (rem, Operation::Multiply(value)))
}

fn parse_test(input: &str) -> IResult<&str, (u64, u64, u64)> {
    tuple((
        preceded(tag("Test: divisible by "), parse_u64),
        preceded(
            multispace1,
            preceded(tag("If true: throw to monkey "), parse_u64),
        ),
        preceded(
            multispace1,
            preceded(tag("If false: throw to monkey "), parse_u64),
        ),
    ))(input.trim())
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .flat_map(|monkey_str| parse_monkey(monkey_str).map(|(_rem, monkey)| monkey))
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    let mut monkeys = parse(input);
    let divider: u64 = monkeys.iter().map(|monkey| monkey.test_divisor).product();

    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let mut curr_monkey = std::mem::take(&mut monkeys[monkey_idx]);
            curr_monkey.throw(&mut monkeys[..], true, divider);
            monkeys[monkey_idx] = curr_monkey;
        }
    }

    monkeys
        .into_iter()
        .map(|monkey| monkey.thrown_items_counter)
        .n_highest::<2>()
        .product()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    let mut monkeys = parse(input);
    let divider: u64 = monkeys.iter().map(|monkey| monkey.test_divisor).product();

    for _round in 0..10_000 {
        for monkey_idx in 0..monkeys.len() {
            let mut curr_monkey = std::mem::take(&mut monkeys[monkey_idx]);
            curr_monkey.throw(&mut monkeys[..], false, divider);
            monkeys[monkey_idx] = curr_monkey;
        }
    }

    monkeys
        .into_iter()
        .map(|monkey| monkey.thrown_items_counter)
        .n_highest::<2>()
        .product()
}
