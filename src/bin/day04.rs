//! # Day 4: Printing Department
//! <https://adventofcode.com/2025/day/4>
//!
//! From an input file modelling a discrete 2d grid, compute:
//!
//! 1. The number of tiles that are accessible in the initial state;
//!
//! 2. The total count of items removed from the grid (cleared tiles) until a steady state
//!    is reached, no more removals possible.
//!
//! A tile is considered accessible or with removable contents if it has fewer than 4
//! occupied neighbours across its 8 adjacent positions.
use aoc_tools::{grid::Grid, pair::Pair};
use std::fs;

type Index = Pair<usize>;

pub fn main() {
    let input = fs::read_to_string("inputs/day04").unwrap();
    let mut grid = parse_input(&input);

    // --- Part 1 ---
    let to_remove = collect_accessible(&grid);
    println!("answer pt1: {}", to_remove.len());

    // --- Part 2 ---
    let mut total_removed = to_remove.len();
    clear_tiles(&mut grid, &to_remove);
    loop {
        let to_remove = collect_accessible(&grid);
        if to_remove.is_empty() {
            break;
        }
        total_removed += to_remove.len();
        clear_tiles(&mut grid, &to_remove);
    }
    println!("answer pt2: {}", total_removed);
}

/// Parses ASCII input into a bordered grid representation.
///
/// 1-cell empty border eliminates out-of-bounds checks for 8-neighborhood scans
///
/// Tile symbols:
/// '.': empty;
/// '@': occupied.
fn parse_input(input: &str) -> Grid<Tile> {
    let len_x = input.lines().next().unwrap().len();

    let iter = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter_map(Tile::parse_char);

    Grid::with_borders(len_x, Tile::Empty, 1, iter)
}

/// Returns a list of all coordinates currently accessible for removal.
///
/// A tile qualify for removal if it has fewer than 4 occupied neighbours.
fn collect_accessible(grid: &Grid<Tile>) -> Vec<Index> {
    let len_x = grid.len_x - 1;
    let len_y = grid.len_y - 1;

    (1..len_x)
        .flat_map(|x| (1..len_y).map(move |y| Index::new(x, y)))
        .filter(|&idx| is_accessible(grid, idx))
        .collect::<Vec<Index>>()
}

/// Checks whether an occupied tile at `index` has fewer than 4 occupied neighbors.
fn is_accessible(grid: &Grid<Tile>, index: Index) -> bool {
    if grid[index] != Tile::Occupied {
        return false;
    }

    #[rustfmt::skip]
    const NEIGHBOUR_OFFSETS: [Pair<i32>; 8] = [
        Pair::new(-1, -1), Pair::new(0, -1), Pair::new(1, -1),
        Pair::new(-1,  0),                        Pair::new(1,  0),
        Pair::new(-1,  1), Pair::new(0,  1), Pair::new(1,  1),
    ];

    let origin = Pair::new(index.x as i32, index.y as i32);

    let count = NEIGHBOUR_OFFSETS
        .iter()
        .map(|offset| origin + offset)
        .map(|i| grid[(i.x as usize, i.y as usize)])
        .filter(|&v| v == Tile::Occupied)
        .count();

    count < 4
}

/// Clears the marked tiles from the grid by setting them to `Tile::Empty`.
fn clear_tiles(grid: &mut Grid<Tile>, to_remove: &[Index]) {
    to_remove.iter().for_each(|&idx| grid[idx] = Tile::Empty);
}

/// Represents the contents of a grid cell.
#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Occupied,
    Empty,
}

impl Tile {
    fn parse_char(c: char) -> Option<Tile> {
        match c {
            '.' => Some(Tile::Empty),
            '@' => Some(Tile::Occupied),
            _ => None,
        }
    }
}
