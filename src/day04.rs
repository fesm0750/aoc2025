//! # Day 4: Printing Department
//! https://adventofcode.com/2025/day/4

use std::fs;

use aoc_tools::{grid::Grid, pair::Pair};

pub fn run() {
    let input = fs::read_to_string("inputs/day04").unwrap();
    let len_x = input.lines().next().unwrap().len();

    let iter = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter_map(Tile::parse_char);

    let grid = Grid::with_borders(len_x, Tile::Empty, 1, iter);

    println!("answer pt1: {}", pt1(&grid));
}

fn pt1(grid: &Grid<Tile>) -> u64 {
    let mask: [Pair<i32>; 8] = [
        Pair::from_tuple((-1, -1)),
        Pair::from_tuple((0, -1)),
        Pair::from_tuple((1, -1)),
        Pair::from_tuple((-1, 0)),
        Pair::from_tuple((1, 0)),
        Pair::from_tuple((-1, 1)),
        Pair::from_tuple((0, 1)),
        Pair::from_tuple((1, 1)),
    ];

    let len_x = grid.len_x - 1;
    let len_y = grid.len_y - 1;

    let mut count_accessible = 0;
    for x in 1..len_x {
        for y in 1..len_y {
            if grid[(x, y)] == Tile::PaperRoll {
                let count = mask
                    .iter()
                    .map(|&m| m + Pair::new(x as i32, y as i32))
                    .map(|i| grid[(i.x as usize, i.y as usize)])
                    .filter(|&v| v == Tile::PaperRoll)
                    .count();

                if count < 4 {
                    count_accessible += 1;
                }
            }
        }
    }

    count_accessible as u64
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    PaperRoll,
    Empty,
}

impl Tile {
    fn parse_char(c: char) -> Option<Tile> {
        match c {
            '.' => Some(Tile::Empty),
            '@' => Some(Tile::PaperRoll),
            _ => None,
        }
    }
}
