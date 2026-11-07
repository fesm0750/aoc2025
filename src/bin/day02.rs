//! Day 2 using numeric IDs.
use aoc2025::Runner;
use aoc2025::day02;

fn main() {
    Runner::init("numeric", "inputs/day02", day02::parse)
        .part("pt1", |ids| day02::numeric::solve_pt1(ids))
        .part("pt2", |ids| day02::numeric::solve_pt2(ids))
        .finish();
}
