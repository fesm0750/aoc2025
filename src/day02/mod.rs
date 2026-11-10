//! Shared implementations and consistence tests for Day 2.

pub mod arrays;
pub mod numeric;
pub mod strings;

pub fn parser(input: &str) -> Vec<u64> {
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
    use crate::day02;

    use super::{arrays, numeric, strings};

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

    #[test]
    fn implementations_agree_pt1() {
        let expected = numeric::solve_pt1(&day02::parser(INPUT));
        assert_eq!(strings::solve_pt1(&strings::parser(INPUT)), expected);
        assert_eq!(arrays::solve_pt1(&arrays::parser(INPUT)), expected);
    }

    #[test]
    fn implementations_agree_pt2() {
        let expected = numeric::solve_pt2(&day02::parser(INPUT));
        assert_eq!(strings::solve_pt2(&strings::parser(INPUT)), expected);
        assert_eq!(arrays::solve_pt2(&arrays::parser(INPUT)), expected);
    }
}
