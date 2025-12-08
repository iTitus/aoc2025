use crate::common::{ParseVecError, Vec3i, parse_lines, parse_vec};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec3i> {
    struct I(Vec3i);

    impl FromStr for I {
        type Err = ParseVecError<ParseIntError>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(I(parse_vec(s)?))
        }
    }

    let v: Vec<I> = parse_lines(input).unwrap();
    v.into_iter().map(|i| i.0).collect()
}

fn sq_dist(a: Vec3i, b: Vec3i) -> u64 {
    a.x.abs_diff(b.x).pow(2) + a.y.abs_diff(b.y).pow(2) + a.z.abs_diff(b.z).pow(2)
}

#[aoc(day8, part1)]
pub fn part1(input: &[Vec3i]) -> usize {
    part1_impl(input, 1000)
}

pub fn part1_impl(input: &[Vec3i], mut max_connections: usize) -> usize {
    assert!(max_connections > 0);
    let sorted_edges: Vec<_> = (0..input.len())
        .tuple_combinations()
        .sorted_unstable_by_key(|&(a, b)| sq_dist(input[a], input[b]))
        .collect();

    // very inefficient union-find data structure for minimum-spanning-tree
    let mut circuits: Vec<_> = (0..input.len()).collect();
    for (a, b) in sorted_edges {
        let old_circuit = circuits[b];
        let new_circuit = circuits[a];
        if old_circuit != new_circuit {
            circuits
                .iter_mut()
                .filter(|c| **c == old_circuit)
                .for_each(|c| *c = new_circuit);
        }

        max_connections -= 1;
        if max_connections == 0 {
            break;
        }
    }

    let counts = circuits.into_iter().counts();
    counts.values().k_largest(3).product()
}

#[aoc(day8, part2)]
pub fn part2(input: &[Vec3i]) -> i64 {
    let sorted_edges: Vec<_> = (0..input.len())
        .tuple_combinations()
        .sorted_unstable_by_key(|&(a, b)| sq_dist(input[a], input[b]))
        .collect();

    // very inefficient union-find data structure for minimum-spanning-tree
    let mut circuits: Vec<_> = (0..input.len()).collect();
    for (a, b) in sorted_edges {
        let old_circuit = circuits[b];
        let new_circuit = circuits[a];
        if old_circuit != new_circuit {
            circuits
                .iter_mut()
                .filter(|c| **c == old_circuit)
                .for_each(|c| *c = new_circuit);
            if circuits.iter().all_equal() {
                return input[a].x * input[b].x;
            }
        }
    }

    panic!("no solution");
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"162,817,812
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
425,690,689"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1_impl(&input_generator(INPUT), 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 25272);
    }
}
