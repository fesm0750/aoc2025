//! # Day 7: Laboratories
//! https://adventofcode.com/2025/day/7

use std::fs;

use Manifold::*;
use aoc_tools::grid::Grid;

pub fn run() {
    let input = fs::read_to_string("inputs/day07").unwrap();
    let len_x = input.split_once('\n').unwrap().0.len();

    let vec: Vec<Manifold> = input.chars().flat_map(Manifold::from_char).collect();
    let mut field = Grid::with_borders(len_x, Empty, 1, vec);

    shoot_beam(&mut field);
    println!("answer pt1: {}", count_splits(&field));
}

fn shoot_beam(field: &mut Grid<Manifold>) {
    for y in 1..field.len_y - 1 {
        for x in 1..field.len_x - 1 {
            let curr = field[(x, y)];
            let below = field[(x, y + 1)];
            match (curr, below) {
                (Start, _) => *field.get_mut(x, y + 1) = Beam,
                (Beam, Empty) => *field.get_mut(x, y + 1) = Beam,
                (Beam, Splitter) => {
                    *field.get_mut(x - 1, y + 1) = Beam;
                    *field.get_mut(x + 1, y + 1) = Beam;
                }
                _ => (),
            }
        }
    }
}

fn count_splits(field: &Grid<Manifold>) -> u64 {
    let mut total = 0;
    for x in 1..field.len_x {
        for y in 1..field.len_y {
            if field[(x, y)] == Splitter && field[(x, y - 1)] == Beam {
                total += 1;
            }
        }
    }
    total
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Manifold {
    Beam,
    Empty,
    Splitter,
    Start,
}

impl Manifold {
    fn from_char(c: char) -> Option<Manifold> {
        Some(match c {
            '.' => Empty,
            '^' => Splitter,
            'S' => Start,
            '|' => Beam,
            _ => None?,
        })
    }
}
