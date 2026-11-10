//! # Day 6: Trash Compactor
//! <https://adventofcode.com/2025/day/6>
use std::{error::Error, fs, str::FromStr, vec};

pub fn main() {
    let input = fs::read_to_string("inputs/day06").unwrap();
    let (list, ops) = parse_pt1::<4>(&input);
    println!("answer pt1: {}", do_math(&list, &ops));

    let (list, ops) = parse_pt2::<4>(&input);
    println!("answer pt1: {}", do_math(&list, &ops));
}

fn parse_ops(ops: &str) -> Vec<Operation> {
    ops.split_whitespace().flat_map(str::parse).collect()
}

fn parse_pt1<const N: usize>(input: &str) -> (Vec<[u64; N]>, Vec<Operation>) {
    let mut lines = input.lines().peekable();
    let x = lines.peek().unwrap().split_whitespace().count();

    let mut list: Vec<[u64; N]> = vec![[0; N]; x];
    for (i, l) in lines.by_ref().take(N).enumerate() {
        for (j, n) in l.split_whitespace().enumerate() {
            list[j][i] = n.parse().unwrap();
        }
    }

    (list, parse_ops(lines.next().unwrap()))
}

fn parse_pt2<const N: usize>(input: &str) -> (Vec<[u64; N]>, Vec<Operation>) {
    let mut input = input.lines();
    let lines: Vec<&str> = input.by_ref().take(N).collect();

    let mut blocks: Vec<[u64; N]> = Vec::new();
    let mut block: Vec<u64> = Vec::new();

    // helper closure to convert the individual digits into a value
    let digits_to_number = |digits: [Option<u8>; N]| {
        digits
            .iter()
            .flatten()
            .rev()
            .enumerate()
            .map(|(i, &d)| d as u64 * 10u64.pow(i as u32))
            .sum::<u64>()
    };

    // helper closure to load a block and avoid duplication of logic
    let mut flush_block = |block: &[u64]| {
        let mut arr = [0; N];
        arr[..block.len()].copy_from_slice(block);
        blocks.push(arr);
    };

    let len = lines[0].len();
    for i in 0..len {
        // creates a column of digits
        let mut column = [None; N];
        for (j, line) in lines.iter().enumerate() {
            column[j] = line[i..i + 1].parse::<u8>().ok();
        }

        if column.iter().any(Option::is_some) {
            let v = digits_to_number(column);
            block.push(v);
        } else {
            flush_block(&block);
            block.clear();
        }
    }
    flush_block(&block);

    (blocks, parse_ops(input.next().unwrap()))
}

fn do_math<const N: usize>(list: &[[u64; N]], ops: &[Operation]) -> u64 {
    assert_eq!(list.len(), ops.len(), "Input arguments are not of the same length.");

    ops.iter()
        .enumerate()
        .map(|(i, op)| {
            let vals = list[i].iter().filter(|&&n| n != 0);
            match op {
                Operation::Sum => vals.sum::<u64>(),
                Operation::Multiplication => vals.product::<u64>(),
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_p1() {
        let (list, ops) = parse_pt1::<3>(INPUT);
        assert_eq!(4277556, do_math(&list, &ops));
    }

    #[test]
    fn test_p2() {
        let (list, ops) = parse_pt2::<3>(INPUT);
        assert_eq!(3263827, do_math(&list, &ops));
    }
}
