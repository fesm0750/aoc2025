use std::fs;

type Range = (u64, u64);

pub fn run() {
    let input = fs::read_to_string("inputs/day05").unwrap();
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<Range> = ranges
        .lines()
        .filter_map(|s| s.split_once('-'))
        .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
        .collect();
    ranges.sort_unstable();

    let ids: Vec<u64> = ids.lines().flat_map(str::parse).collect();

    println!("answer pt1: {}", count_fresh(&ids, &ranges));
    println!("answer pt2: {}", flat_ranges(&ranges).count());
}

fn is_fresh(id: &u64, ranges: &[Range]) -> bool {
    ranges.iter().any(|(min, max)| min <= id && id <= max)
}

fn count_fresh(ids: &[u64], ranges: &[Range]) -> u64 {
    ids.iter().filter(|id| is_fresh(id, ranges)).count() as u64
}

fn flat_ranges(sorted_ranges: &[Range]) -> impl Iterator {
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
        assert_eq!(14, flat_ranges(&ranges).count());
    }
}
