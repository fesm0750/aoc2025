//! Shared entry point and implementations for Day 2.

use std::time::Instant;

pub mod arrays;
pub mod numeric;
pub mod strings;

pub fn run(solve: fn(&str) -> (u64, u64)) {
    let input = std::fs::read_to_string("inputs/day02").unwrap();
    let start = Instant::now();
    let (pt1, pt2) = solve(&input);
    let duration = start.elapsed();

    println!("answer pt1: {pt1}");
    println!("answer pt2: {pt2}");
    println!("Time elapsed: {duration:?}");
}

pub fn parse(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            start..=end
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{arrays, numeric, strings};

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

    // #[test]
    // fn implementations_agree() {
    //     let expected = numeric::solve(INPUT);
    //     assert_eq!(strings::solve(INPUT), expected);
    //     assert_eq!(arrays::solve(INPUT), expected);
    // }
}
