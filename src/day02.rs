use std::fs;

type Range = (u64, u64);

pub fn run() {
    let input = fs::read_to_string("inputs/day02").unwrap();
    let ranges = parse_input(&input);

    println!("answer pt1: {}", pt1_bf(&ranges));
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(',')
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect::<Vec<Range>>()
}

/// Brute force approach.
///
/// Creates every product ID and tests if the first half of the ID is equals to the second
/// half.
fn pt1_bf(ranges: &[Range]) -> u64 {
    ranges
        .iter()
        .flat_map(|range| range.0..range.1 + 1)
        .filter(|&v| {
            let num_digits = v.ilog10() + 1;
            let half_divider = 10u64.pow(num_digits / 2);
            num_digits % 2 == 0 && v / half_divider == v % half_divider
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
";

    #[test]
    fn test() {
        let ranges = parse_input(INPUT);
        let total = pt1_bf(&ranges);
        assert_eq!(total, 1227775554);
    }
}
