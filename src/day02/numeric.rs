//! Numeric implementation: expand ranges into `u64` IDs.

pub fn solve_pt1(ids: &[u64]) -> u64 {
    ids.iter().filter(|&&id| is_repeated_twice(id)).sum()
}

pub fn solve_pt2(ids: &[u64]) -> u64 {
    ids.iter().filter(|&&id| is_repeating_pattern(id)).sum()
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
        let reconstructed = (0..digits / length)
            .map(|position| pattern * 10u64.pow(length * position))
            .sum::<u64>();
        id == reconstructed
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
        let ids = crate::day02::parse(INPUT);
        assert_eq!(solve_pt1(&ids), 1227775554);
        assert_eq!(solve_pt2(&ids), 4174379265);
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
