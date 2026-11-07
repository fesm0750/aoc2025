//! Day 2 using fixed-size digit arrays.
use aoc2025::Runner;
use aoc2025::day02;

fn main() {
    Runner::init("arrays", "inputs/day02", day02::arrays::parse)
        .part("pt1", |ids| day02::arrays::solve_pt1(ids))
        .part("pt2", |ids| day02::arrays::solve_pt2(ids))
        .finish();
}
