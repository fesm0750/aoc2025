//! Day 2 using string IDs.
use aoc2025::Runner;
use aoc2025::day02;

fn main() {
    Runner::init("strings", "inputs/day02", day02::strings::parser)
        .part1(|ids| day02::strings::solve_pt1(ids))
        .part2(|ids| day02::strings::solve_pt2(ids))
        .finish();
}
