//! Numeric implementation: expand ranges into `u64` IDs.

pub fn solve(input: &str) -> (u64, u64) {
    let ids = parse(input);
    (
        ids.iter().filter(|&&id| is_repeated_twice(id)).sum(),
        ids.iter().filter(|&&id| is_repeating_pattern(id)).sum(),
    )
}

fn parse(input: &str) -> Vec<u64> {
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

fn is_repeated_twice(id: u64) -> bool {
    let digits = id.ilog10() + 1;
    let divider = 10u64.pow(digits / 2);
    digits % 2 == 0 && id / divider == id % divider
}

fn is_repeating_pattern(id: u64) -> bool {
    let digits = id.ilog10() + 1;
    (1..=digits / 2).filter(|&length| digits % length == 0).any(|length| {
        let pattern = id % 10u64.pow(length);
        (0..digits / length)
            .map(|position| pattern * 10u64.pow(length * position))
            .sum::<u64>()
            == id
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

    #[test]
    fn solves_both_parts() {
        assert_eq!(solve(INPUT), (1227775554, 4174379265));
    }

    #[test]
    fn checks_repeating_patterns() {
        assert!(is_repeated_twice(55));
        assert!(!is_repeated_twice(555));
        assert!(is_repeating_pattern(555));
        assert!(is_repeating_pattern(765765765));
        assert!(!is_repeating_pattern(561165));
    }
}
