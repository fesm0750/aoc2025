//! # Day 01: Secret Entrance
//! https://adventofcode.com/2025/day/1
//!
//! ## Problem Statement
//!
//! From an input file containing a sequence of rotation commands to be applied at the
//! `Dial` of a safe, compute:
//!
//! 1. The number of times the dial is left pointing at 0 after any rotation is performed;
//!
//! 2. The number of times any click causes the dial to point at 0, regardless of whether
//!    it happens during a rotation or at the end of one.
//!
//! The `Dial` comprises 100 positions [0, 100) and starts at 50.
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day01").unwrap();
    let rotations = parse_input(&input);

    let mut dial1 = Dial::new();
    println!("answer pt1: {}", solve_pt1(&mut dial1, &rotations));

    let mut dial2 = Dial::new();
    println!("answer pt2: {}", solve_pt2(&mut dial2, &rotations));
}

/// Parses a list of rotation commands from an ASCII string, returning a `Vec<i16>` where
/// each element represents the length of a rotation.
///
/// - Rotations for the `R`ight are returned as positive values.
/// - Rotations for the `L`eft are returned as negative values.
///
/// # Arguments
/// - `input`: An ASCII string where each line contains a rotation command. Each command
///   starts with either `'L'` or `'R'` followed by a positive number. For example:
///   `"R16"` or `"L100"`.
///
/// # Assumptions
/// - The input is guaranteed to be ASCII.
/// - The numeric part of each command is always positive.
///
/// # Panics
/// - The input string is empty;
/// - A line does not start with `'L'` or `'R'`;
/// - The numeric part cannot be parsed into an `i16`.
///
/// # Examples
/// ```
/// let input = "R16\nL100\nR5";
/// let rotations = parse_rotations(input).unwrap();
/// assert_eq!(rotations, vec![16, -100, 5]);
/// ```
fn parse_input(input: &str) -> Vec<i16> {
    input
        .lines()
        .map(|s| {
            let dir = match &s[0..1] {
                "L" => -1,
                "R" => 1,
                _ => panic!("Wrong Direction (expected L or R)."),
            };
            s[1..].parse::<i16>().unwrap() * dir
        })
        .collect()
}

/// Applies a sequence of rotations and returns the total of times a rotation ended up
/// at zero.
fn solve_pt1(dial: &mut Dial, rotations: &[i16]) -> usize {
    rotations.iter().filter(|&&r| dial.rotate(r)).count()
}

/// Applies a sequence of rotations and returns the total of times it passed by zero,
/// regardless of whether it happens during a rotation or at the end of one.
fn solve_pt2(dial: &mut Dial, rotations: &[i16]) -> u16 {
    rotations.iter().map(|&r| dial.rotate_2(r)).sum()
}

/// Represents a circular dial.
///
/// # Range and Wrapping
/// - The dial conceptually has 100 positions `[0, 99]` and starts at position `50`.
/// - To simplify arithmetic, positions may also be stored as negative values. The
///   effective range is `[-99, 99]`.
/// - Values outside this range are wrapped back into it:
///   - `-100` and `100` both wrap to `0`.
///   - Other values wrap to their signed equivalent, e.g.:
///     - `220` → `20`
///     - `-220` → `-20`
///     
/// This design allows straightforward addition and subtraction of offsets without
/// requiring too much special-case handling for wrap-around.
struct Dial {
    pos: i16,
}

impl Dial {
    const SIZE: i16 = 100; // dial has positions [0..=99]
    const START: i16 = 50; // dial begins pointing at position 50

    fn new() -> Dial {
        Dial { pos: Dial::START }
    }

    /// Applies a given rotation to the dial and returns whether it ended up at zero
    /// (according to the **part 1** rules).
    fn rotate(&mut self, r: i16) -> bool {
        self.pos = (self.pos + r) % Self::SIZE;

        self.pos == 0
    }

    /// Applies a given rotation to the dial and returns the number of times it passed by
    /// zero, regardless of whether it happens during a rotation or at the end of one
    /// (according to the **part 2** rules).
    fn rotate_2(&mut self, r: i16) -> u16 {
        let old_pos = self.pos;
        let new_pos = self.pos + r;
        self.pos = new_pos % Self::SIZE;

        (new_pos / Self::SIZE).unsigned_abs()
            + if old_pos != 0 && new_pos.signum() * old_pos.signum() <= 0 {
                1 // zero crossing correction
            } else {
                0
            }
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

    #[test]
    fn test_parse_input() {
        let input = parse_input(INPUT);
        assert_eq!(input, ROTATIONS);
    }

    #[test]
    fn test_dial_solve_pt1() {
        let mut dial = Dial::new();
        assert_eq!(solve_pt1(&mut dial, &ROTATIONS), 3);
    }

    #[test]
    fn test_dial_solve_pt2() {
        let mut dial = Dial::new();
        assert_eq!(solve_pt2(&mut dial, &ROTATIONS), 6);
    }

    /// Small rotations wrapping.
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
        assert_eq!(dial.rotate_2(-110), 1, "wrap past left, negative to negative"); // -120 -> 20
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
