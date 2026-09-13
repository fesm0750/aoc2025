//! # Day 01: Secret Entrance
//! <https://adventofcode.com/2025/day/1>
//!
//! ## Problem Statement
//!
//! From an input file containing a sequence of rotation commands for a safe
//! `Dial`, compute:
//!
//! 1. The number of times the dial is left pointing at 0 after any rotation is performed;
//!
//! 2. The number of times any click causes the dial to point at 0, regardless of whether
//!    it happens during a rotation or at the end of one.
//!
//! The `Dial` comprises 100 positions [0, 100) and starts at 50.
//!
//! ## Implementation Details
//!
//! ### State Representation: Symmetric Signed Modulo
//!
//! Tracks dial in signed range (-100, 100) instead of canonical [0, 100), avoiding
//! Euclidean modulo overhead. This works because the target position zero is uniquely
//! stable in both Euclidean and symmetric forms, enabling direct use of the `%` operator.
//!
//! - Rotating Right adds positive offsets; Left adds negative.
use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day01").unwrap();
    let rotations = parse_input(&input);

    println!("answer pt1: {}", solve_pt1(&rotations));
    println!("answer pt2: {}", solve_pt2(&rotations));
}

/// Parses a list of signed rotation offsets from ASCII input.
///
/// Each line must contain a command consisting of a direction prefix and a
/// magnitude; 'R' corresponds to positive angular displacement, whereas 'L'
/// maps to a negative one.
///
/// # Panics
/// Panics if a line is empty, or if a non-empty line has invalid direction
/// prefixes or invalid numeric payloads.
///
/// # Example
/// ```ignore
/// let input = "R16\nL100\nR5";
/// let rotations = parse_input(input);
/// assert_eq!(rotations, vec![16, -100, 5]);
/// ```
fn parse_input(input: &str) -> Vec<i16> {
    input
        .lines()
        .map(|s| {
            let (dir, mag) = s.split_at(1);
            let dir = match dir {
                "L" => -1,
                "R" => 1,
                _ => panic!("Invalid direction: expected L or R. Found: \"{}\"", dir),
            };
            let mag = mag.parse::<i16>().expect("Invalid rotation magnitude");

            mag * dir
        })
        .collect()
}

/// Computes the number of operations terminating at zero.
fn solve_pt1(rotations: &[i16]) -> usize {
    let mut dial = Dial::new();
    rotations.iter().filter(|&&r| dial.rotate(r)).count()
}

/// Computes the aggregate zero crossings during all rotations.
fn solve_pt2(rotations: &[i16]) -> usize {
    let mut dial = Dial::new();
    rotations.iter().map(|&r| dial.rotate_2(r) as usize).sum()
}

/// Cyclic dial over Z_100, starting at position 50.
///
/// Position is tracked in the symmetric range (-100, 100) to simplify wrap‑around for
/// negatives. Zero is stable in both Euclidean and symmetric representations, allowing
/// direct use of the `%` operator without extra modulo corrections.
struct Dial {
    pos: i16,
}

impl Dial {
    const SIZE: i16 = 100; // [0..=99]
    const START: i16 = 50;

    fn new() -> Dial {
        Dial { pos: Dial::START }
    }

    /// Part 1: Advance dial by `r` clicks and return true if positioned exactly
    /// at 0.
    fn rotate(&mut self, r: i16) -> bool {
        self.pos = (self.pos + r) % Self::SIZE;

        self.pos == 0
    }

    /// Part 2: Advances dial by `r` clicks and returns the number of zero-state
    /// visits.
    ///
    /// # Technical Note
    ///
    /// Due to the use of a symmetric representation, it is necessary to account for the
    /// transition from `old_pos` to `new_pos` across the origin. The correction is
    /// applied if and only if the sign changes or reaches zero, excluding the
    /// non-crossing case where `old_pos == 0`.
    fn rotate_2(&mut self, r: i16) -> u16 {
        let old_pos = self.pos;
        let new_pos = self.pos + r;
        self.pos = new_pos % Self::SIZE;

        // compensation for traversing zero across signs
        let zero_cross = (old_pos != 0 && new_pos.signum() * old_pos.signum() <= 0) as u16;

        (new_pos / Self::SIZE).unsigned_abs() + zero_cross
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    const ROTATIONS: [i16; 10] = [-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];

    // ==========================================
    // AoC test cases
    // ==========================================

    #[test]
    fn test_parse_input() {
        let input = parse_input(INPUT);
        assert_eq!(input, ROTATIONS);
    }

    #[test]
    fn test_solve_pt1() {
        assert_eq!(solve_pt1(&ROTATIONS), 3);
    }

    #[test]
    fn test_solve_pt2() {
        assert_eq!(solve_pt2(&ROTATIONS), 6);
    }

    // ==========================================
    // General Dial tests for rotate2
    // ==========================================

    /// General small rotations.
    #[test]
    fn test_dial_rotate2_small() {
        let mut dial = Dial::new(); // 50

        assert_eq!(dial.rotate_2(40), 0); // 90
        assert_eq!(dial.rotate_2(-80), 0); // 10
        assert_eq!(dial.rotate_2(-20), 1, "cross zero past left"); // -10
        assert_eq!(dial.rotate_2(-80), 0); // -90
        assert_eq!(dial.rotate_2(40), 0); // -50
        assert_eq!(dial.rotate_2(100), 1, "cross zero past right"); // 50
    }

    /// Rotations ending or starting exactly at 0.
    #[test]
    fn test_dial_rotate2_end_start_zero() {
        let mut safe = Dial::new();

        assert_eq!(safe.rotate_2(-50), 1, "goes to zero");
        assert_eq!(safe.rotate_2(1), 0, "get out of zero by the right");
        assert_eq!(safe.rotate_2(99), 1, "right wrap to zero");
        assert_eq!(safe.rotate_2(-1), 0, "get out of zero by the left");
        assert_eq!(safe.rotate_2(-99), 1, "left wrap to zero");
    }

    /// Small rotations wrapping.
    #[test]
    fn test_dial_rotate2_wrapping() {
        let mut dial = Dial::new(); // 50

        assert_eq!(dial.rotate_2(60), 1, "wrap past right, positive to positive"); // 110 -> 10
        assert_eq!(dial.rotate_2(-20), 1, "wrap past left, positive to negative"); // -10
        assert_eq!(dial.rotate_2(-110), 1, "wrap past left, negative to negative"); // -120 -> -20
        assert_eq!(dial.rotate_2(30), 1, "wrap past right, negative to positive"); // 10
    }

    /// Rotations wrapping from zero to zero.
    #[test]
    fn test_dial_rotate2_wrap_zero() {
        let mut dial = Dial::new();

        assert_eq!(dial.rotate_2(-50), 1, "goes to zero");
        assert_eq!(dial.rotate_2(-100), 1, "from zero, left wrap to zero");
        assert_eq!(dial.rotate_2(100), 1, "from zero, right wrap to zero");
    }

    /// Large rotations that cross zero multiple times.
    #[test]
    fn test_dial_rotate2_large() {
        let mut dial = Dial::new(); // 50

        assert_eq!(dial.rotate_2(500), 5, "keep on positive"); // 550 -> 50
        assert_eq!(dial.rotate_2(570), 6, "keep on positive"); // 620 -> 20
        assert_eq!(dial.rotate_2(-670), 7, "goes to negative"); // -650 -> -50
        assert_eq!(dial.rotate_2(-200), 2, "keep on negative"); // -250 -> -50
        assert_eq!(dial.rotate_2(-370), 4, "keep on negative"); // -420 -> -20
        assert_eq!(dial.rotate_2(570), 6, "goes to positive"); // 550 -> 50
    }
}
