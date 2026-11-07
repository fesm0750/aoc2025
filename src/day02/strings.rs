//! String implementation: keep each generated ID in its decimal representation.

use crate::day02;

pub fn parse(path: &str) -> Vec<String> {
    day02::parse(path).iter().map(|id| id.to_string()).collect()
}

pub fn solve_pt1(ids: &[String]) -> u64 {
    ids.iter()
        .filter(|id| is_repeated_twice(id))
        .map(|id| id.parse::<u64>().unwrap())
        .sum()
}

pub fn solve_pt2(ids: &[String]) -> u64 {
    ids.iter()
        .filter(|id| is_repeating_pattern(id))
        .map(|id| id.parse::<u64>().unwrap())
        .sum()
}

fn is_repeated_twice(id: &str) -> bool {
    let (left, right) = id.as_bytes().split_at(id.len() / 2);
    id.len().is_multiple_of(2) && left == right
}

fn is_repeating_pattern(id: &str) -> bool {
    (1..id.len() / 2 + 1)
        .filter(|&length| id.len().is_multiple_of(length))
        .any(|length| {
            id.as_bytes()
                .chunks(length)
                .all(|chunk| chunk == &id.as_bytes()[..length])
        })
}
