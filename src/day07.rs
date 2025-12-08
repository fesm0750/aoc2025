//! # Day 7: Laboratories
//! https://adventofcode.com/2025/day/7

use std::fs;

use Manifold::*;
use aoc_tools::grid::Grid;

pub fn run() {
    let input = fs::read_to_string("inputs/day07").unwrap();
    let mut field = parse_input(&input);

    shoot_beam(&mut field);
    println!("answer pt1: {}", count_splits(&field));
    println!("answer pt2: {}", count_timelines(&field));
}

fn parse_input(input: &str) -> Grid<Manifold> {
    let len_x = input.split_once('\n').unwrap().0.len();
    let vec: Vec<Manifold> = input.chars().flat_map(Manifold::from_char).collect();
    Grid::with_borders(len_x, Empty, 1, vec)
}

fn shoot_beam(field: &mut Grid<Manifold>) {
    for row in 1..field.len_y - 1 {
        for col in 1..field.len_x - 1 {
            let curr = field[(col, row)];
            let below = field.get_mut(col, row + 1);

            let update = |upd: &mut Manifold, n: u64| {
                if let Beam(m) = upd {
                    *upd = Beam(n + *m);
                } else {
                    *upd = Beam(n);
                }
            };

            match (curr, *below) {
                (Start, _) => *below = Beam(1),
                (Beam(n), Empty) => update(below, n),
                (Beam(n), Beam(_)) => update(below, n),
                (Beam(n), Splitter) => {
                    let left = field.get_mut(col - 1, row + 2);
                    update(left, n);

                    let right = field.get_mut(col + 1, row + 2);
                    update(right, n);
                }
                _ => (),
            }
        }
    }
}

fn count_splits(field: &Grid<Manifold>) -> u64 {
    let mut splits = 0;
    for row in 1..field.len_y - 1 {
        for col in 1..field.len_x - 1 {
            let (curr, above) = (field[(col, row)], field[(col, row - 1)]);
            if let Splitter = curr
                && let Beam(_) = above
            {
                splits += 1;
            }
        }
    }
    splits
}

fn count_timelines(field: &Grid<Manifold>) -> u64 {
    let last_row = field.len_y - 1;
    let last_row = field.row(last_row);
    last_row
        .iter()
        .map(|item| if let &Beam(n) = item { n } else { 0 })
        .sum()
}

#[derive(Debug, Clone, Copy)]
enum Manifold {
    Beam(u64),
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
            '|' => Beam(0),
            _ => None?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let mut field = parse_input(input);
        shoot_beam(&mut field);
        assert_eq!(21, count_splits(&field));
        assert_eq!(40, count_timelines(&field));
    }
}
