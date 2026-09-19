//! # Day 5: Cafeteria
//! <https://adventofcode.com/2025/day/5>

use std::fs;

type IdRange = (u64, u64);

pub fn main() {
    let input = fs::read_to_string("inputs/day05").unwrap();
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<IdRange> = ranges
        .lines()
        .filter_map(|s| s.split_once('-'))
        .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
        .collect();
    ranges.sort_unstable();

    let ids: Vec<u64> = ids.lines().flat_map(str::parse).collect();

    println!("answer pt1: {}", count_fresh_ids(&ids, &ranges));
    println!("answer pt2: {}", flatten_ranges(&ranges).count());
}

fn is_fresh_id(id: &u64, ranges: &[IdRange]) -> bool {
    ranges.iter().any(|(left, right)| left <= id && id <= right)
}

fn count_fresh_ids(ids: &[u64], ranges: &[IdRange]) -> u64 {
    ids.iter().filter(|id| is_fresh_id(id, ranges)).count() as u64
}

/// Assumes zero does not belong to any range.
fn flatten_ranges(sorted_ranges: &[IdRange]) -> impl Iterator<Item = u64> {
    let mut last = 0;
    sorted_ranges
        .iter()
        .flat_map(move |&r| {
            let range = if last >= r.1 {
                None
            } else if last >= r.0 {
                Some(last + 1..r.1 + 1)
            } else {
                Some(r.0..r.1 + 1)
            };
            last = last.max(r.1);
            range
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squash_ranges() {
        let mut ranges = [(3, 5), (10, 14), (16, 20), (12, 18)];
        ranges.sort_unstable();
        assert_eq!(14, flatten_ranges(&ranges).count());
    }
}
