//! # Day 01: Secret Entrance
//! https://adventofcode.com/2025/day/1
//!
//! ## Problem Statement
//!
//! From an input file containing a sequence of `Rotation`s to be applied at the dial of a
//! safe, compute:
//!
//! 1. The number of times the dial is left pointing at 0 after any rotation is performed;
//!
//! 2. The number of times any click causes the dial to point at 0, regardless of whether
//!    it happens during a rotation or at the end of one.
use std::{error::Error, str::FromStr};

use aoc_tools::input;

pub fn run() {
    let rotations = input::lines_to_vec::<Rotation>("day01").unwrap();

    let mut safe1 = Dial::new();
    rotations.iter().for_each(|r| safe1.rotate(r));
    println!("answer pt1: {}", safe1.n_zeros);

    let mut safe2 = Dial::new();
    rotations.iter().for_each(|r| safe2.rotate2(r));
    println!("answer pt2: {}", safe2.n_zeros);
}

/// Direction of rotation.
enum Direction {
    Left,
    Right,
}

/// Rotation stores the `Direction` and the number of `clicks` to rotate the `Dial`.
struct Rotation {
    dir: Direction,
    clicks: u16,
}

/// Dial stores the current position and keeps tally of the number of zeros (`n_zeros`).
///
/// The dial is comprised of 100 positions [0..=99] and starts at 50.
struct Dial {
    pos: u16,
    n_zeros: u16,
}

impl Dial {
    const SIZE: u16 = 100; // dial has positions [0..=99]
    const START: u16 = 50; // dial begins pointing at position 50

    fn new() -> Dial {
        Dial {
            pos: Dial::START,
            n_zeros: 0,
        }
    }

    /// Updates the dial for a given rotation, according to the **part 1** rules.
    ///
    /// The number of zeros (`n_zeros`) is incremented each time the dial finishes a
    /// rotation pointing at position 0.
    fn rotate(&mut self, r: &Rotation) {
        self.pos = match r.dir {
            Direction::Right => (self.pos + r.clicks) % Self::SIZE,
            Direction::Left if r.clicks <= self.pos => self.pos - r.clicks,
            Direction::Left => Self::SIZE - self.pos.abs_diff(r.clicks) % Self::SIZE,
        };

        // the second left case above may not wrap 100 to 0 (not a problem for this
        // implementation)
        if self.pos == 0 || self.pos == 100 {
            self.n_zeros += 1;
        }
    }

    /// Updates the dial for a given rotation, according to the **part 2** rules.
    ///
    /// The number of zeros (`n_zeros`) is incremented every time the dial passes by zero,
    /// regardless of whether it happens during a rotation of the end of one.
    fn rotate2(&mut self, r: &Rotation) {
        match r.dir {
            Direction::Right => {
                let pos = self.pos + r.clicks;
                self.n_zeros += pos / Self::SIZE;
                self.pos = pos % Self::SIZE;
            }
            // Does not cross zero
            Direction::Left if r.clicks < self.pos => {
                self.pos -= r.clicks;
            }
            // Rotates past zero
            Direction::Left => {
                let pos = self.pos.abs_diff(r.clicks);

                // When rotating left past 0, we cross zero once.
                // If starting exactly at 0, we don't count that crossing again.
                self.n_zeros += pos / Self::SIZE + if self.pos != 0 { 1 } else { 0 };

                // Taking module again so the value can wrap around properly
                self.pos = (Self::SIZE - pos % Self::SIZE) % Self::SIZE;
            }
        };
    }
}

impl FromStr for Rotation {
    type Err = Box<dyn Error>;

    /// Parses a rotation from a string like "R16" or "L100".
    ///
    /// # Arguments
    /// Input string must start with 'L' or 'R', followed by a u16 number (0–65535).
    ///
    /// # Errors
    /// Returns an error if:
    /// - Input is empty
    /// - First character is not 'L' or 'R'
    /// - The number part fails to parse as u16
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err("Empty Rotation")?
        }

        let dir = match &s[0..1] {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err("Wrong Direction (expected L or R)."),
        }?;
        let clicks = s[1..].parse::<u16>()?;

        Ok(Rotation { dir, clicks })
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

    #[test]
    fn test_dial_rotate() {
        let mut safe1 = Dial::new();
        aoc_tools::parse::lines::<Rotation>(INPUT).for_each(|r| {
            safe1.rotate(&r);
        });
        assert_eq!(safe1.n_zeros, 3);
    }

    #[test]
    fn test_dial_rotate2() {
        let mut safe2 = Dial::new();
        aoc_tools::parse::lines::<Rotation>(INPUT).for_each(|r| {
            safe2.rotate2(&r);
        });
        assert_eq!(safe2.n_zeros, 6);
    }

    // TODO: Add edge case tests:
    // - Rotation starting/ending exactly at 0
    // - Very large clicks that wrap multiple times
    // - Left rotations that cross 0
}
