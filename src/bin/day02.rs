//! # Day 2: Gift Shop
//! <https://adventofcode.com/2025/day/2>
//!
//! ## Implementation details
//!
//! Total number of generated ids = 2,593,147
use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day02").unwrap();
    let ids = parse_input(&input);
    // println!("number of elements: {}", ids.len());

    println!("answer pt1: {}", solve_pt1(&ids));
    println!("answer pt2: {}", solve_pt2(&ids));
}

/// Parses a list of ranges from an ASCII input into a list of `ID`s.
fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(',')
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse::<u64>().unwrap())
        })
        .flat_map(|range| range.0..range.1 + 1)
        .collect::<Vec<u64>>()
}

/// Returns the sum of invalid `IDs` according to part 1.
///
/// Assumes no ID is zero, otherwise panics.
fn solve_pt1(ids: &[u64]) -> u64 {
    ids.iter().filter(|&&id| is_repeated_twice(id)).sum::<u64>()
}

fn solve_pt2(ids: &[u64]) -> u64 {
    ids.iter().filter(|&&id| is_repeating_pattern(id)).sum::<u64>()
}

/// Checks whether the first half of an `id` is equal to its second half.
///
/// `id` cannot be zero, otherwise panis.
fn is_repeated_twice(id: u64) -> bool {
    let num_digits = id.ilog10() + 1;
    let half_divider = 10u64.pow(num_digits / 2);
    num_digits % 2 == 0 && id / half_divider == id % half_divider
}

/// Checks whether the id is composed of a repeating pattern.
///
/// `id` cannot be zero, otherwise panis.
fn is_repeating_pattern(id: u64) -> bool {
    let num_digits = id.ilog10() + 1;

    for pat_len in 1..num_digits / 2 + 1 {
        // checks if the length of the pattern fits the number
        if num_digits % pat_len > 0 {
            continue;
        }

        // construct a number from the pattern
        // let len = id.ilog10() as u64 + 1;
        let pat = id % 10u64.pow(pat_len);
        let constructed = (0..num_digits / pat_len)
            .map(|n| pat * 10u64.pow(pat_len * n))
            .sum::<u64>();

        // check if the built repeated pattern matches the id
        if constructed == id {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    #[test]
    fn test_solve_pt1() {
        let ids = parse_input(INPUT);
        let total = solve_pt1(&ids);
        assert_eq!(total, 1227775554);
    }

    #[test]
    fn test_solve_pt2() {
        let ids = parse_input(INPUT);
        let total = solve_pt2(&ids);
        assert_eq!(total, 4174379265);
    }
}
