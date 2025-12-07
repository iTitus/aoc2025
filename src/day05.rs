use crate::common::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
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

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<Range>, Vec<u64>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    (parse_lines(ranges).unwrap(), parse_lines(ids).unwrap())
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Range>, Vec<u64>)) -> usize {
    input
        .1
        .iter()
        .filter(|id| {
            input
                .0
                .iter()
                .any(|range| (range.0..=range.1).contains(id))
        })
        .count()
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<Range>, Vec<u64>)) -> u64 {
    let mut ranges = vec![];
    let mut current = None;
    for r in input.0.iter().copied().sorted_unstable_by_key(|r| r.0) {
        if r.0 > r.1 {
            continue;
        }

        let Some(cur) = &mut current else {
            current = Some(r);
            continue;
        };

        debug_assert!(r.0 >= cur.0);
        if r.0 <= (cur.1 + 1) {
            cur.1 = cur.1.max(r.1);
        } else {
            ranges.push(*cur);
            current = Some(r);
        }
    }
    ranges.extend(current);

    ranges.into_iter().map(|r| r.1 - r.0 + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 14);
    }
}
