//! # Day 3: Lobby
//! <https://adventofcode.com/2025/day/3>

use std::cmp::Ordering::*;

use aoc2025::Runner;

const BANK_SIZE: usize = 100;
const SUB_SIZE_PT1: usize = 2;
const SUB_SIZE_PT2: usize = 12;

pub fn main() {
    Runner::init("", "inputs/day03", parser::<BANK_SIZE>)
        .part1(|banks| total_joltage::<BANK_SIZE, SUB_SIZE_PT1>(banks))
        .part2(|banks| total_joltage::<BANK_SIZE, SUB_SIZE_PT2>(banks))
        .finish();
}

fn parser<const N: usize>(input: &str) -> Vec<[u8; N]> {
    input
        .lines()
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

fn max_subsequence<const N: usize, const M: usize>(bank: &[u8; N]) -> [u8; M] {
    let mut sub = [0u8; M];
    sub.copy_from_slice(&bank[N - M..N]);

    for &curr in bank[0..N - M].iter().rev() {
        let mut aux = curr;
        for b in sub.iter_mut() {
            match aux.cmp(b) {
                Greater => std::mem::swap(&mut *b, &mut aux),
                Less => break,
                Equal => continue,
            }
        }
    }

    sub
}

fn total_joltage<const N: usize, const M: usize>(banks: &[[u8; N]]) -> u64 {
    let digits_to_integer = |d: [u8; M]| {
        d.iter()
            .rev()
            .enumerate()
            .map(|(i, &d)| d as u64 * 10u64.pow(i as u32))
            .sum::<u64>()
    };

    banks.iter().map(max_subsequence).map(digits_to_integer).sum()
}

/// Original pt1 function specialized for a subsequence of size 2
#[allow(dead_code)]
fn pt1_old<const N: usize>(banks: &[[u8; N]]) -> u64 {
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

    const LINE_LEN: usize = 15;

    #[test]
    fn test_total_joltage() {
        let ranges = parser::<LINE_LEN>(INPUT);
        assert_eq!(357, total_joltage::<LINE_LEN, SUB_SIZE_PT1>(&ranges)); // pt1 test
        assert_eq!(3121910778619, total_joltage::<LINE_LEN, SUB_SIZE_PT2>(&ranges)); // pt2 test
    }

    #[test]
    fn test_pt1_old() {
        let ranges = parser::<LINE_LEN>(INPUT);
        let total = pt1_old(&ranges);
        assert_eq!(total, 357);
    }
}
