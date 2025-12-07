//! # Day 6: Trash Compactor
//! https://adventofcode.com/2025/day/6

use std::{error::Error, fs, str::FromStr};
pub fn run() {
    let input = fs::read_to_string("inputs/day06").unwrap();
    let (list, ops) = parse_input(&input);

    println!("answer pt1: {}", do_math(&list, &ops));
}

fn parse_input(input: &str) -> (Vec<[u64; 4]>, Vec<Operation>) {
    let mut lines = input.lines().peekable();
    let x = lines.peek().unwrap().split_whitespace().count();

    let mut list: Vec<[u64; 4]> = vec![Default::default(); x];
    for (i, l) in lines.by_ref().take(4).enumerate() {
        for (j, n) in l.split_whitespace().enumerate() {
            list[j][i] = n.parse().unwrap();
        }
    }

    let ops = lines.next().unwrap().split_whitespace().flat_map(str::parse).collect();

    (list, ops)
}

fn do_math<const N: usize>(list: &[[u64; N]], ops: &[Operation]) -> u64 {
    ops.iter()
        .enumerate()
        .map(|(i, op)| {
            let vals = list[i];
            match op {
                Operation::Sum => vals.iter().sum::<u64>(),
                Operation::Multiplication => vals.iter().product::<u64>(),
            }
        })
        .sum()
}

enum Operation {
    Sum,
    Multiplication,
}

impl FromStr for Operation {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Operation::Sum,
            "*" => Operation::Multiplication,
            _ => Err("Invalid Operation")?,
        })
    }
}
