//! Array implementation: represent each ID as a fixed-size digit array.

use crate::day02;

#[derive(Clone, Copy)]
pub struct Id {
    digits: [u8; Self::MAX_DIGITS],
    len: usize,
}

pub fn parser(input: &str) -> Vec<Id> {
    day02::parser(input).iter().map(Id::from_value).collect()
}

pub fn solve_pt1(ids: &[Id]) -> u64 {
    ids.iter()
        .filter(|id| is_repeated_twice(id))
        .map(|id| id.value())
        .sum()
}

pub fn solve_pt2(ids: &[Id]) -> u64 {
    ids.iter()
        .filter(|id| is_repeating_pattern(id))
        .map(|id| id.value())
        .sum()
}

fn is_repeated_twice(id: &Id) -> bool {
    let digits = id.digits();
    let half = digits.len() / 2;
    digits.len().is_multiple_of(2) && digits[half..] == digits[..half]
}

fn is_repeating_pattern(id: &Id) -> bool {
    let digits = id.digits();
    let len = digits.len();
    (1..len / 2 + 1)
        .filter(|&length| len.is_multiple_of(length))
        .any(|length| digits.chunks(length).all(|chunk| chunk == &digits[..length]))
}

// ==========================================
// Struct Implementation
// ==========================================

impl Id {
    pub const MAX_DIGITS: usize = 20;

    fn from_value(value: &u64) -> Self {
        let text = value.to_string();
        let mut digits = [0; Self::MAX_DIGITS];
        let offset = Self::MAX_DIGITS - text.len();
        for (index, digit) in text.bytes().enumerate() {
            digits[offset + index] = digit - b'0';
        }
        Self {
            digits,
            len: text.len(),
        }
    }

    fn value(self) -> u64 {
        self.digits[Self::MAX_DIGITS - self.len..]
            .iter()
            .fold(0, |value, &digit| value * 10 + u64::from(digit))
    }

    fn digits(&self) -> &[u8] {
        &self.digits[Self::MAX_DIGITS - self.len..]
    }
}
