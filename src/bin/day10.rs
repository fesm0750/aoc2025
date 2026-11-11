use std::mem;

use aoc2025::Runner;

const MAX_BUTTONS: usize = 10;

type Data = Vec<(u16, Vec<u16>, Vec<u8>)>;

pub fn main() {
    Runner::init("name", "inputs/day10", parser).part1(part1).finish();
}

fn parser(input: &str) -> Data {
    let mut machines = Data::new();

    // helper closure to `fold` a bit stream into a number
    let fold_bits_to_number = |acc: u16, bit: bool| (acc << 1) | bit as u16;

    for line in input.lines() {
        let mut vals = line.split_ascii_whitespace().peekable();

        let lights = vals.next().unwrap();
        let light = lights
            .strip_circumfix('[', ']')
            .unwrap()
            .as_bytes()
            .iter()
            .rev()
            .map(|&bit| match bit {
                b'#' => true,
                b'.' => false,
                _ => panic!("Wrong input character for light."),
            })
            .fold(0u16, fold_bits_to_number);

        let mut buttons = Vec::<u16>::new();
        while let "(" = &vals.peek().unwrap()[0..1] {
            let mut button = [false; MAX_BUTTONS];
            vals.next()
                .unwrap()
                .strip_circumfix('(', ')')
                .unwrap()
                .as_bytes()
                .iter()
                .filter(|&&c| c != b',')
                .map(|&c| c - b'0')
                .for_each(|b| button[b as usize] = true);

            let button = button.iter().rev().copied().fold(0u16, fold_bits_to_number);
            buttons.push(button);
        }

        let joltages = vals
            .next()
            .unwrap()
            .strip_circumfix('{', '}')
            .unwrap()
            .split(',')
            .flat_map(str::parse)
            .collect::<Vec<u8>>();

        machines.push((light, buttons, joltages));
    }

    machines
}

// brute force using a breadth approach
// because each button press is equivalent to a xor operation, only the first press is
// effective and the pressing order is irrelevant.
fn part1(machines: &Data) -> u64 {
    let mut max_pushes = 0;

    let total_pushes = machines
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let pushes = configure_machine(m.0, &m.1, i);
            max_pushes = max_pushes.max(pushes);
            pushes
        })
        .sum();

    println!("\nDebug max generations: {max_pushes}");
    total_pushes
}

// brute force approach generating all possible states from button presses until reaching
// solution.
// Note for the future: as each button pressing equivales to a xor operation, no need to
// press a button more than once or to consider order of operation
fn configure_machine(lights: u16, buttons: &[u16], mach_id: usize) -> u64 {
    let mut generation = 0;
    let mut gen_cache0 = vec![0];
    let mut gen_cache1 = Vec::<u16>::new();

    loop {
        generation += 1;
        if generation > 9 {
            println!("Error Machine {mach_id}: Permutations above allowable generational limit.");
            break generation;
        }

        for &v in gen_cache0.iter() {
            for &b in buttons {
                let state = v ^ b;
                if state == lights {
                    return generation;
                }
                gen_cache1.push(state);
            }
        }

        mem::swap(&mut gen_cache0, &mut gen_cache1);
        gen_cache1.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test() {
        let machines = parser(INPUT);
        assert_eq!(7, part1(&machines));
    }
}
