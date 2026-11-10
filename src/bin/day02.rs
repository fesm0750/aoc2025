//! # Day 2: Gift Shop
//! <https://adventofcode.com/2025/day/2>
//!
//! ## Problem Statement
//!
//! Given a list of numeric `ID` ranges in ASCII input, identify invalid values formed by
//! repeating digit patterns.
//!
//! 1. Return the sum of `ID`s consisting of a digit sequence repeated exactly twice.
//!
//! 2. Return the sum of `ID`s consisting of any pattern repeated two or more times.
//!
//! Invalid IDs must consist solely of repeating patterns, with no other digits allowed.
//!
//! Assumes the input ranges does not overlap.
//!
//! ## Implementation Details
//!
//! A brute-force approach generating the full list of IDs from the given ranges and
//! testing each one against the invalidity conditions. From the provided input, the total
//! number of ids is 2,593,147.
//!
//! For part 2, subsequences of the original ID are extracted and repeated to reconstruct
//! a candidate number, which is then compared against the original. Constructing numbers
//! through successive multiplications is preferred over repeated divisions, as it is
//! both simpler and more efficient.
//!
//! ## Performance
//!
//! This version process the id in numeric form. Another version processing the id as a
//! string was also experimented with. Although part2 using string is faster, the
//! generation of the id list as string is much more expensive.
//!
//! |       | numeric | string | array  |
//! | ----- | ------- | ------ | ------ |
//! | parse |  23 ms  | 152 ms | 113 ms |
//! | pt1   |  11 ms  |  11 ms |  11 ms |
//! | pt2   |  75 ms  |  51 ms |  47 ms |
//! | total | 110 ms  | 214 ms | 172 ms | *note: ceiling has been applied
use aoc2025::Runner;
use aoc2025::day02;

fn main() {
    Runner::init("numeric", "inputs/day02", day02::parse)
        .part("pt1", |ids| day02::numeric::solve_pt1(ids))
        .part("pt2", |ids| day02::numeric::solve_pt2(ids))
        .finish();
}
