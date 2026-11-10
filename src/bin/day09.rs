use aoc_tools::pair::Pair;
use aoc2025::Runner;

type Point = Pair<u32>;

pub fn main() {
    Runner::init("", "inputs/day09", parser)
        .part1(|points| find_rectangle(points))
        .finish();
}

fn parser(input: &str) -> Vec<Point> {
    input.lines().flat_map(str::parse).collect()
}

fn find_rectangle(points: &[Point]) -> u64 {
    let area = |p0: &Point, p1: &Point| (p0.x.abs_diff(p1.x) + 1) as u64 * (p0.y.abs_diff(p1.y) + 1) as u64;

    let mut max_area = 0;
    for (i, n) in points.iter().enumerate() {
        for m in points.iter().take(i) {
            max_area = max_area.max(area(n, m));
        }
    }

    max_area
}
