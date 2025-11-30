use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::iter;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[(u32, u32)]) -> u32 {
    let a = input.iter().map(|(n, _)| n).sorted_unstable().collect_vec();
    let b = input.iter().map(|(_, n)| n).sorted_unstable().collect_vec();
    iter::zip(a, b).map(|(a, b)| a.abs_diff(*b)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[(u32, u32)]) -> usize {
    let counts = input.iter().map(|(_, n)| n).counts();

    let mut score = 0;
    for &(n, _) in input {
        score += n as usize * counts.get(&n).copied().unwrap_or_default()
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 31);
    }
}
