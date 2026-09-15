//! # Day 2: Gift Shop
//! <https://adventofcode.com/2025/day/2>
//!
//! ## Implementation details
//!
use std::{fs, time::Instant};

pub fn main() {
    let input = fs::read_to_string("inputs/day02").unwrap();

    let start = Instant::now();
    let ids = parse_input(&input);
    let t0 = start.elapsed();
    let pt1 = solve_pt1(&ids);
    let t1 = start.elapsed();
    let pt2 = solve_pt2(&ids);
    let duration = start.elapsed();

    println!("answer pt1: {}, at {:?}", pt1, t1 - t0);
    println!("answer pt2: {}, at {:?}", pt2, duration - t1);
    println!("parse time: {:?}", t0);
    println!("Time elapsed: {:?}", duration);
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
    (1..num_digits / 2 + 1)
        .filter(|&len| num_digits % len == 0)
        .any(|len| {
            let pattern = id % 10u64.pow(len);
            let candidate = (0..num_digits / len)
                .map(|n| pattern * 10u64.pow(len * n))
            .sum::<u64>();
            candidate == id
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    // ==========================================
    // AoC test cases
    // ==========================================

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

    // ==========================================
    // General test cases
    // ==========================================

    #[test]
    fn test_is_repeated_twice() {
        assert!(is_repeated_twice(55), "single digit pattern");
        assert!(is_repeated_twice(56105610), "more digits");
        assert!(is_repeating_pattern(12345671234567), "larger pattern");

        // false test cases
        assert!(!is_repeated_twice(555), "even number of digits");
        assert!(!is_repeated_twice(565656), "more than twice");
        assert!(!is_repeated_twice(561165), "mirrored");
        assert!(!is_repeated_twice(5610561), "zero at start");
        assert!(!is_repeated_twice(5615610), "broken zero at end");
    }

    #[test]
    fn test_is_repeating_pattern() {
        assert!(is_repeating_pattern(55), "single digit twice");
        assert!(is_repeating_pattern(555), "single digit three times");
        assert!(is_repeating_pattern(56565656), "multiple, even number of digits");
        assert!(is_repeating_pattern(765765765), "multiple, odd number of digits");
        assert!(is_repeating_pattern(12345671234567), "larger pattern");

        // false test cases
        assert!(!is_repeating_pattern(561165), "mirrored");
        assert!(!is_repeating_pattern(56156561), "middle pattern broken");
        assert!(!is_repeating_pattern(5610561), "zero at start");
        assert!(!is_repeating_pattern(5615610), "broken zero at end");
    }
}
