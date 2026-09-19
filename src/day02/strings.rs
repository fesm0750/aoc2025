//! String implementation: keep each generated ID in its decimal representation.

pub fn solve(input: &str) -> (u64, u64) {
    let ids = parse(input);
    (
        ids.iter()
            .filter(|id| is_repeated_twice(id))
            .map(|id| id.parse::<u64>().unwrap())
            .sum(),
        ids.iter()
            .filter(|id| is_repeating_pattern(id))
            .map(|id| id.parse::<u64>().unwrap())
            .sum(),
    )
}

fn parse(input: &str) -> Vec<String> {
    input
        .trim()
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start..=end).map(|id| id.to_string()).collect::<Vec<_>>()
        })
        .collect()
}

fn is_repeated_twice(id: &str) -> bool {
    let (left, right) = id.split_at(id.len() / 2);
    id.len() % 2 == 0 && left == right
}

fn is_repeating_pattern(id: &str) -> bool {
    (1..=id.len() / 2)
        .filter(|length| id.len() % length == 0)
        .any(|length| {
            id.as_bytes()
                .chunks(length)
                .all(|chunk| chunk == &id.as_bytes()[..length])
        })
}
