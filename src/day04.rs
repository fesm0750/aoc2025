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

    let mut grid = Grid::with_borders(len_x, Tile::Empty, 1, iter);

    let mut n_removed = count_removable(&mut grid).unwrap();
    remove(&mut grid);
    println!("answer pt1: {}", n_removed);

    while let Some(x) = count_removable(&mut grid) {
        n_removed += x;
        remove(&mut grid);
    }
    println!("answer pt2: {}", n_removed);
}

fn count_removable(grid: &mut Grid<Tile>) -> Option<u64> {
    let len_x = grid.len_x - 1;
    let len_y = grid.len_y - 1;

    let count = (1..len_x)
        .flat_map(|x| (1..len_y).map(move |y| Pair::new(x, y)))
        .filter(|&idx| mark_removable(grid, idx))
        .count() as u64;

    if count > 0 { Some(count) } else { None }
}

fn mark_removable(grid: &mut Grid<Tile>, index: Pair<usize>) -> bool {
    if grid[index] != Tile::PaperRoll {
        return false;
    }

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

    let count = mask
        .iter()
        .map(|&m| m + Pair::new(index.x as i32, index.y as i32))
        .map(|i| grid[(i.x as usize, i.y as usize)])
        .filter(|&v| v == Tile::PaperRoll || v == Tile::ToBeRemoved)
        .count();

    if count < 4 {
        *grid.get_mut(index.x, index.y) = Tile::ToBeRemoved;
        true
    } else {
        false
    }
}

fn remove(grid: &mut Grid<Tile>) {
    grid.iter_mut()
        .filter(|t| **t == Tile::ToBeRemoved)
        .for_each(|t| *t = Tile::Empty);
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    PaperRoll,
    ToBeRemoved,
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
