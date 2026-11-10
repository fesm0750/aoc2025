//! # Day 5: Cafeteria
//! <https://adventofcode.com/2025/day/5>
use std::cmp::Ordering::Equal;

use aoc2025::Runner;

type IdRange = (u64, u64);
type Data = (Vec<IdRange>, Vec<u64>);

pub fn main() {
    Runner::init("", "inputs/day05", parser)
        .part1(|data| count_fresh_ids(&data.0, &data.1))
        .part2(|data| flatten_ranges_and_count(&data.0))
        .finish();
}

fn parser(raw: &str) -> Data {
    let (ranges, ids) = raw.split_once("\n\n").unwrap();
    let ids: Vec<u64> = ids.lines().flat_map(str::parse).collect();

    let mut ranges: Vec<IdRange> = ranges
        .lines()
        .filter_map(|s| s.split_once('-'))
        .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
        .collect();
    ranges.sort_unstable();

    (ranges, ids)
}

fn count_fresh_ids(ranges: &[IdRange], ids: &[u64]) -> u64 {
    let is_fresh_id = |id| ranges.iter().any(|(left, right)| left <= id && id <= right);
    ids.iter().filter(|id| is_fresh_id(id)).count() as u64
}

/// Generates an iterator with all unique `ids` belonging to the given ranges.
/// Assumes zero does not belong to any range.
fn flatten_ranges_and_count(sorted_ranges: &[IdRange]) -> u64 {
    assert!(sorted_ranges.is_sorted(), "ranges are not sorted.");

    let mut last = 0; // last generated element
    sorted_ranges
        .iter()
        .flat_map(move |&r| {
            // ranges are sorted, current first element is always equal or
            // greater than the previous first
            let range = if r.0 > last {
                Some(r.0..r.1 + 1) // new unique range
            } else if r.1 > last {
                Some(last + 1..r.1 + 1) // intersection, generate remaining
            } else {
                None // subset, already generated
            };

            last = last.max(r.1);
            range
        })
        .flatten()
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squash_ranges() {
        let mut ranges = [(3, 5), (10, 14), (16, 20), (12, 18)]; // new range, subset
        ranges.sort_unstable();
        assert_eq!(14, flatten_ranges_and_count(&ranges));

        let sorted = [(3, 5), (10, 16), (12, 20), (14, 18)]; // new range, intersection, subsect
        assert_eq!(14, flatten_ranges_and_count(&sorted));
    }
}
