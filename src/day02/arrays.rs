//! Array implementation: represent each ID as a fixed-size digit array.

const MAX_DIGITS: usize = 20;

#[derive(Clone, Copy)]
struct Id {
    digits: [u8; MAX_DIGITS],
    len: usize,
}

pub fn solve(input: &str) -> (u64, u64) {
    let ids = parse(input);
    (
        ids.iter()
            .filter(|id| is_repeated_twice(id))
            .map(|id| id.value())
            .sum(),
        ids.iter()
            .filter(|id| is_repeating_pattern(id))
            .map(|id| id.value())
            .sum(),
    )
}

fn parse(input: &str) -> Vec<Id> {
    input
        .trim()
        .split(',')
        .flat_map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            (start..=end).map(Id::from_value).collect::<Vec<_>>()
        })
        .collect()
}

impl Id {
    fn from_value(value: u64) -> Self {
        let text = value.to_string();
        let mut digits = [0; MAX_DIGITS];
        let offset = MAX_DIGITS - text.len();
        for (index, digit) in text.bytes().enumerate() {
            digits[offset + index] = digit - b'0';
        }
        Self {
            digits,
            len: text.len(),
        }
    }

    fn value(self) -> u64 {
        self.digits[MAX_DIGITS - self.len..]
            .iter()
            .fold(0, |value, &digit| value * 10 + u64::from(digit))
    }

    fn digits(&self) -> &[u8] {
        &self.digits[MAX_DIGITS - self.len..]
    }
}

fn is_repeated_twice(id: &Id) -> bool {
    let digits = id.digits();
    let half = digits.len() / 2;
    digits.len() % 2 == 0 && digits[..half] == digits[half..]
}

fn is_repeating_pattern(id: &Id) -> bool {
    let digits = id.digits();
    (1..=digits.len() / 2)
        .filter(|length| digits.len() % length == 0)
        .any(|length| digits.chunks(length).all(|chunk| chunk == &digits[..length]))
}
