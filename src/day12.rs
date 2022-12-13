use pathfinding::directed::bfs::bfs;
use std::{ops::Index, str::FromStr};

const START_POS: (usize, usize) = (20, 0);
const END_POS: (usize, usize) = (20, 43);

#[derive(Debug)]
struct HeatMap {
    rows: usize,
    cols: usize,
    values: Vec<i8>,
}

impl Index<(usize, usize)> for HeatMap {
    type Output = i8;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.values[row * self.cols + col]
    }
}

impl FromStr for HeatMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s.find("\n").unwrap();
        let values: Vec<i8> = s
            .lines()
            .flat_map(str::chars)
            .map(|c| match c {
                'S' => -1,
                'E' => (b'z' - b'a' + 1) as i8,
                c @ 'a'..='z' => c as i8 - b'a' as i8,
                _ => unreachable!(),
            })
            .collect();
        let rows = values.len() / cols;
        Ok(Self { cols, rows, values })
    }
}

fn adjacents(
    pos: (usize, usize),
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let (i, j) = pos;
    const CARDINALS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    CARDINALS
        .into_iter()
        .flat_map(move |(r, c)| {
            (i as isize + r)
                .try_into()
                .and_then(|i| (j as isize + c).try_into().map(|j| (i, j)))
        })
        .filter(move |pos| pos.0 < rows && pos.1 < cols)
}

#[aoc_generator(day12)]
#[inline(always)]
fn generator(input: &str) -> HeatMap {
    HeatMap::from_str(input).unwrap()
}

#[aoc(day12, part1)]
fn part1(input: &HeatMap) -> usize {
    bfs(
        &END_POS,
        |&curr_pos| {
            adjacents(curr_pos, input.rows, input.cols).filter(move |pos| {
                let starting_height = input[curr_pos];
                let dest_height = input[*pos];
                starting_height - dest_height <= 1
            })
        },
        |pos| *pos == START_POS,
    )
    .map(|path| path.len())
    .unwrap() - 1
}

#[aoc(day12, part2)]
fn part2(input: &HeatMap) -> usize {
    bfs(
        &END_POS,
        |&curr_pos| {
            adjacents(curr_pos, input.rows, input.cols).filter(move |pos| {
                let starting_height = input[curr_pos];
                let dest_height = input[*pos];
                starting_height - dest_height <= 1
            })
        },
        |pos| input[*pos] == 0,
    )
    .map(|path| path.len())
    .unwrap() - 1

}
