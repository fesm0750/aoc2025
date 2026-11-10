use std::{error::Error, str::FromStr, vec};

use aoc2025::Runner;
use rustc_hash::FxHashMap;

type Data = (Vec<Point3d>, Vec<(u64, usize, usize)>);

type HashMap<K, V> = FxHashMap<K, V>;

pub fn main() {
    Runner::init("", "inputs/day08", parser)
        .part1(|data| part1(&data.1, 1000))
        .part2(|data| part2(&data.1, &data.0))
        .finish();
}

struct Point3d {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

fn parser(input: &str) -> Data {
    let coords = parse_coords(input);
    let distances = distances(&coords);

    (coords, distances)
}

fn parse_coords(input: &str) -> Vec<Point3d> {
    input.lines().flat_map(str::parse).collect()
}

fn distances(coords: &[Point3d]) -> Vec<(u64, usize, usize)> {
    let mut list = Vec::new();
    for (i, n) in coords.iter().enumerate() {
        for (j, m) in coords.iter().take(i).enumerate() {
            list.push((square_distance(n, m), i, j));
        }
    }
    list.sort_unstable();

    list
}

fn part1(sorted_distances: &[(u64, usize, usize)], n_connections: usize) -> u64 {
    assert!(sorted_distances.is_sorted(), "Distances are not sorted.");

    let mut boxes: HashMap<usize, usize> = HashMap::default(); // k box, v circuit
    let mut circuits: Vec<Vec<usize>> = Vec::new();

    sorted_distances
        .iter()
        .take(n_connections)
        .for_each(|&(_, a, b)| connect(a, b, &mut boxes, &mut circuits));

    let mut lengths: Vec<u64> = circuits.iter().map(|c| c.len() as u64).collect();
    lengths.sort_unstable();
    lengths.into_iter().rev().take(3).product::<u64>()
}

fn part2(sorted_distances: &[(u64, usize, usize)], coords: &[Point3d]) -> u64 {
    assert!(sorted_distances.is_sorted(), "Distances are not sorted.");

    let mut boxes: HashMap<usize, usize> = HashMap::default(); // k box, v circuit
    let mut circuits: Vec<Vec<usize>> = Vec::new();

    for &(_, a, b) in sorted_distances.iter() {
        connect(a, b, &mut boxes, &mut circuits);

        if circuits.iter().any(|c| c.len() == coords.len()) {
            return coords[a].x as u64 * coords[b].x as u64;
        }
    }

    0
}

fn connect(a: usize, b: usize, boxes: &mut HashMap<usize, usize>, circuits: &mut Vec<Vec<usize>>) {
    // check if the junction boxes already belong to a circuit
    match (boxes.get(&a), boxes.get(&b)) {
        // if both are not, create a new circuit and add them
        (None, None) => {
            circuits.push(vec![a, b]);
            let idx = circuits.len() - 1;
            boxes.insert(a, idx);
            boxes.insert(b, idx);
        }
        // if just one is, add the other one
        (None, Some(&idx)) => {
            circuits[idx].push(a);
            boxes.insert(a, idx);
        }
        (Some(&idx), None) => {
            circuits[idx].push(b);
            boxes.insert(b, idx);
        }
        // if both are, merge the circuits
        (Some(&idxa), Some(&idxb)) => {
            if idxa == idxb {
                return;
            }

            let circ_b = circuits[idxb].clone();
            circuits[idxa].extend(circ_b.iter());
            circ_b.into_iter().for_each(|i| {
                boxes.entry(i).and_modify(|e| *e = idxa);
            });
            // boxes.values_mut().filter(|v| **v == idxb).for_each(|v| *v = idxa);
            circuits[idxb].clear();
        }
    }
}

fn square_distance(p0: &Point3d, p1: &Point3d) -> u64 {
    let dx = p0.x.abs_diff(p1.x) as u64;
    let dy = p0.y.abs_diff(p1.y) as u64;
    let dz = p0.z.abs_diff(p1.z) as u64;

    dx * dx + dy * dy + dz * dz
}

impl FromStr for Point3d {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(',').flat_map(str::parse);

        Ok(Point3d {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test() {
        let coords = parse_coords(INPUT);
        let list: Vec<(u64, usize, usize)> = distances(&coords);
        assert_eq!(40, part1(&list, 10));
        assert_eq!(25272, part2(&list, &coords));
    }
}
