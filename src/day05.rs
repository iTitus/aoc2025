use crate::common::parse_lines;
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
                .any(|range| (range.0..=range.1).contains(&id))
        })
        .count()
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<Range>, Vec<u64>)) -> u64 {
    let mut ranges: Vec<Range> = vec![];
    let mut to_add = input.0.clone();
    'outer: while let Some(mut r) = to_add.pop() {
        for existing in &mut ranges {
            // full inclusion
            if r.0 > r.1 || r.0 >= existing.0 && r.1 <= existing.1 {
                continue 'outer;
            }

            // other way inclusion
            if existing.0 > r.0 && existing.1 < r.1 {
                to_add.push(Range(existing.1 + 1, r.1));
                r.1 = existing.0 - 1;
            }

            // a little bit of overlap one side
            if (existing.0..=existing.1).contains(&r.0) {
                assert!(existing.1 < u64::MAX);
                r.0 = existing.1 + 1;
            }

            // a little bit of overlap on the other side
            if (existing.0..=existing.1).contains(&r.1) {
                assert!(existing.0 > 0);
                r.1 = existing.0 - 1;
            }
        }
        ranges.push(r);
    }

    ranges.iter().map(|r| r.1 - r.0 + 1).sum()
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
