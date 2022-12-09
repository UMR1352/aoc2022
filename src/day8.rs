use std::ops::Index;

use itertools::{Itertools, FoldWhile::{Continue, Done}};

#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Tree(u8);

impl From<char> for Tree {
    fn from(c: char) -> Self {
        match c {
            '0'..='9' => Self(c as u8 - b'0'),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    trees: Vec<Vec<Tree>>,
}

impl Map {
    pub fn new(trees: Vec<Vec<Tree>>) -> Self {
        let width = trees[0].len();
        let height = trees.len();
        Self {
            trees,
            width,
            height,
        }
    }
    pub fn contains_pos(&self, pos: (usize, usize)) -> bool {
        let (x, y) = pos;
        x < self.width && y < self.height
    }
    pub fn perimeter(&self) -> usize {
        self.height * 2 + (self.width - 2) * 2
    }
}

impl Index<&(usize, usize)> for Map {
    type Output = Tree;
    fn index(&self, index: &(usize, usize)) -> &Self::Output {
        let (x, y) = *index;
        &self.trees[y][x]
    }
}

type Dir = (i32, i32);

const CARDINALS: [Dir; 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

#[derive(Debug)]
struct Line {
    dir: &'static Dir,
    pos: (i32, i32),
}

impl Line {
    pub const fn new(pos: (usize, usize), dir: &'static Dir) -> Self {
        Self {
            pos: (pos.0 as i32, pos.1 as i32),
            dir,
        }
    }
}

impl Iterator for Line {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.pos.0 += self.dir.0;
        self.pos.1 += self.dir.1;

        self.pos
            .0
            .try_into()
            .and_then(|x| self.pos.1.try_into().map(|y| (x, y)))
            .ok()
    }
}

#[aoc_generator(day8)]
fn generator(input: &str) -> Map {
    let trees = input
        .lines()
        .map(|line| line.chars().map(Into::into).collect::<Vec<Tree>>())
        .collect();

    Map::new(trees)
}

#[aoc(day8, part1)]
fn part1(input: &Map) -> usize {
    let trees_on_border = input.perimeter();
    (1..input.width - 1)
        .flat_map(|x| (1..input.height - 1).map(move |y| (x, y)))
        .filter(|center| {
            CARDINALS
                .iter()
                .any(|dir| {
                    Line::new(*center, dir)
                        .take_while(|pos| input.contains_pos(*pos))
                        .map(|pos| &input[&pos])
                        .all(|height| *height < input[center])
                })
        })
        .count()
        + trees_on_border
}

#[aoc(day8, part2)]
fn part2(input: &Map) -> usize {
    (0..input.width)
        .flat_map(|x| (0..input.height).map(move |y| (x, y)))
        .map(|center| {
            CARDINALS
                .iter()
                .map(|dir| {
                    Line::new(center, dir)
                        .fold_while(0, |visibility, pos| {
                            if !input.contains_pos(pos) {
                                Done(visibility)
                            } else if input[&pos] < input[&center] {
                                Continue(visibility + 1)
                            } else {
                                Done(visibility + 1)
                            }
                        }).into_inner()
                })
                .product::<usize>()
        })
        .max()
        .unwrap()
}