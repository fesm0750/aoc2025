//! # Day 3: Lobby
//! https://adventofcode.com/2025/day/3

use std::fs;

const BANK_SIZE: usize = 100;

pub fn run() {
    let input = fs::read_to_string("inputs/day03").unwrap();
    let banks = parse_input::<BANK_SIZE>(&input);

    println!("answer pt1: {}", pt1(&banks));
}

fn parse_input<const N: usize>(s: &str) -> Vec<[u8; N]> {
    s.lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect::<Vec<u8>>()
                .try_into()
                .expect("Array was not completely filled")
        })
        .collect()
}

fn pt1<const N: usize>(banks: &[[u8; N]]) -> u64 {
    let mut total_joltage = 0;
    for bank in banks {
        let (mut n1, mut n0) = (0, 0);

        for &b in &bank[0..N - 1] {
            n0 = n0.max(b);
            if n0 > n1 {
                n1 = n0;
                n0 = 0;
            }
        }

        n0 = n0.max(bank[N - 1]);
        total_joltage += (n1 * 10 + n0) as u64;
    }

    total_joltage
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_pt1() {
        let ranges = parse_input::<15>(INPUT);
        let total = pt1(&ranges);
        assert_eq!(total, 357);
    }
    }
