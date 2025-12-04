//! # Day 3: Lobby
//! https://adventofcode.com/2025/day/3

use std::fs;

const BANK_SIZE: usize = 100;
type BatteryBank = [u8; BANK_SIZE];

pub fn run() {
    let input = fs::read_to_string("inputs/day03").unwrap();
    let banks = parse_input(&input);

    println!("answer pt1: {}", pt1(&banks));
}

fn parse_input(s: &str) -> Vec<BatteryBank> {
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

fn pt1(banks: &[BatteryBank]) -> u64 {
    let mut total_joltage = 0;
    for bank in banks {
        let (mut n1, mut n0) = (0, 0);

        for &b in &bank[0..BANK_SIZE - 1] {
            n0 = n0.max(b);
            if n0 > n1 {
                n1 = n0;
                n0 = 0;
            }
        }

        n0 = n0.max(bank[BANK_SIZE - 1]);
        total_joltage += (n1 * 10 + n0) as u64;
    }

    total_joltage
}
