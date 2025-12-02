use crate::common::parse_split;
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Range(pub u64, pub u64);

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(())?;
        Ok(Self(
            start.parse().map_err(|_| ())?,
            end.parse().map_err(|_| ())?,
        ))
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Range> {
    parse_split(input, ',').unwrap()
}

fn is_repeated_twice(n: u64) -> bool {
    let s = n.to_string();
    if !s.len().is_multiple_of(2) {
        return false;
    }

    let (prefix, suffix) = s.split_at(s.len() / 2);
    prefix == suffix
}

#[aoc(day2, part1)]
pub fn part1(input: &[Range]) -> u64 {
    input
        .iter()
        .flat_map(|r| r.0..=r.1)
        .filter(|&n| is_repeated_twice(n))
        .sum()
}

fn is_repeated_at_least_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    for part_len in 1..=(len / 2) {
        if !len.is_multiple_of(part_len) {
            continue;
        }

        let (prefix, mut rest) = s.split_at(part_len);
        while let Some(rest_) = rest.strip_prefix(prefix) {
            if rest_.is_empty() {
                return true;
            }

            rest = rest_;
        }
    }

    false
}

#[aoc(day2, part2)]
pub fn part2(input: &[Range]) -> u64 {
    input
        .iter()
        .flat_map(|r| r.0..=r.1)
        .filter(|&n| is_repeated_at_least_twice(n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 4174379265);
    }
}
