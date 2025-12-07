use std::fs;

type Range = (u64, u64);

pub fn run() {
    let input = fs::read_to_string("inputs/day05").unwrap();
    let (ranges, ids) = input.split_once("\n\n").unwrap();

    let ranges: Vec<Range> = ranges
        .lines()
        .filter_map(|s| s.split_once('-'))
        .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
        .collect();

    let ids: Vec<u64> = ids.lines().flat_map(str::parse).collect();

    println!("answer pt1: {}", count_fresh(&ids, &ranges));
}

fn is_fresh(id: &u64, ranges: &[Range]) -> bool {
    ranges.iter().any(|(min, max)| min <= id && id <= max)
}

fn count_fresh(ids: &[u64], ranges: &[Range]) -> u64 {
    ids.iter().filter(|id| is_fresh(id, ranges)).count() as u64
}
