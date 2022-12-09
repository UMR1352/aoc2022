use itertools::Either;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{alpha1, char, digit1},
    combinator::{map, map_res, opt, recognize},
    multi::many1,
    sequence::{pair, preceded, separated_pair},
    Finish, IResult,
};

#[derive(Debug)]
enum Command<'i> {
    Ls,
    CdBack,
    CdInto(&'i str),
}

#[derive(Debug)]
struct File {
    size: usize,
}

fn parse_file_or_command(input: &str) -> IResult<&str, Either<File, Command>> {
    alt((
        map(parse_file, Either::Left),
        map(parse_command, Either::Right),
    ))(input)
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    alt((parse_ls, parse_cd))(input)
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    preceded(tag("$ "), tag("ls"))(input).map(|(rem, _)| (rem, Command::Ls))
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    map_res(
        separated_pair(
            preceded(tag("$ "), tag("cd")),
            char(' '),
            alt((tag("/"), tag(".."), recognize(many1(alpha1)))),
        ),
        |(_cmd, arg)| {
            Ok::<Command<'_>, ()>(
                if arg == ".." {
                    Command::CdBack
                } else {
                    Command::CdInto(arg)
                }
            )
        },
    )(input)
}

fn parse_file(input: &str) -> IResult<&str, File> {
    map_res(
        separated_pair(parse_file_size, char(' '), parse_file_name),
        |(size, _name)| Ok::<File, ()>(File { size }),
    )(input)
}

fn parse_file_size(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), |size: &str| size.parse::<usize>())(input)
}

fn parse_file_name(input: &str) -> IResult<&str, &str> {
    recognize(pair(alpha1, opt(pair(char('.'), alpha1))))(input)
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    let (mut all_dirs, mut stack) = input
        .lines()
        .filter(|line| !line.starts_with("dir"))
        .filter_map(|line| {
            parse_file_or_command(line)
                .finish()
                .map(|(_, file_cmd)| file_cmd)
                .ok()
        })
        .fold(
            (vec![], vec![]),
            |(mut all_dirs, mut stack), file_or_cmd| match file_or_cmd {
                Either::Left(file) => {
                    *stack.last_mut().unwrap() += file.size;
                    (all_dirs, stack)
                }
                Either::Right(Command::CdInto(_)) => {
                    stack.push(0);
                    (all_dirs, stack)
                }
                Either::Right(Command::CdBack) => {
                    let curr_dir_size = stack.pop();
                    *stack.last_mut().unwrap() += curr_dir_size.unwrap();
                    all_dirs.push(curr_dir_size.unwrap());
                    (all_dirs, stack)
                }
                _ => (all_dirs, stack),
            },
        );
    let root_size = stack.iter_mut().rev().fold(0, |prev, curr_size| {
        *curr_size += prev;
        *curr_size
    });
    all_dirs.push(root_size);
    all_dirs
        .into_iter()
        .chain(stack.into_iter())
        .filter(|x| *x <= 100_000)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    let (all_dirs, mut stack) = input
        .lines()
        .filter(|line| !line.starts_with("dir"))
        .filter_map(|line| {
            parse_file_or_command(line)
                .finish()
                .map(|(_, file_cmd)| file_cmd)
                .ok()
        })
        .fold(
            (vec![], vec![]),
            |(mut all_dirs, mut stack), file_or_cmd| match file_or_cmd {
                Either::Left(file) => {
                    *stack.last_mut().unwrap() += file.size;
                    (all_dirs, stack)
                }
                Either::Right(Command::CdInto(_)) => {
                    stack.push(0);
                    (all_dirs, stack)
                }
                Either::Right(Command::CdBack) => {
                    let curr_dir_size = stack.pop();
                    *stack.last_mut().unwrap() += curr_dir_size.unwrap();
                    all_dirs.push(curr_dir_size.unwrap());
                    (all_dirs, stack)
                }
                _ => (all_dirs, stack),
            },
        );
    let root_size = stack.iter_mut().rev().fold(0, |prev, curr_size| {
        *curr_size += prev;
        *curr_size
    });
    let available_space = 70_000_000 - root_size;
    all_dirs
        .into_iter()
        .chain(stack.into_iter())
        .filter(|x| available_space + *x >= 30_000_000)
        .min()
        .unwrap()
}