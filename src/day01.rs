use crate::common::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

const WHEEL_SIZE: u32 = 100;
const STARTING_POS: u32 = 50;
const _: () = assert!(STARTING_POS < WHEEL_SIZE);
const _: () = assert!(WHEEL_SIZE <= i32::MAX as u32);

#[derive(Debug, Copy, Clone)]
pub struct Rotation(pub i32);

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let neg = s.starts_with('L');
        let rest = s.strip_prefix(['L', 'R']).ok_or(())?;
        let res: i32 = rest.parse().map_err(|_| ())?;
        Ok(Self(if neg { -res } else { res }))
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Rotation> {
    parse_lines(input).unwrap()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Rotation]) -> usize {
    let mut amount = 0;
    let mut n = STARTING_POS;
    for r in input {
        n = (n as i32).wrapping_add(r.0).rem_euclid(WHEEL_SIZE as i32) as u32;
        if n == 0 {
            amount += 1;
        }
    }

    amount
}

#[aoc(day1, part2)]
pub fn part2(input: &[Rotation]) -> usize {
    let mut amount = 0;
    let mut n = STARTING_POS;
    for r in input {
        if r.0 == 0 {
            continue;
        }

        if n > 0 {
            let abs = r.0.unsigned_abs();
            amount += (abs / WHEEL_SIZE) as usize;
            let rest = abs % WHEEL_SIZE;
            let diff = if r.0 >= 0 { WHEEL_SIZE - n } else { n };
            if rest >= diff {
                amount += 1;
            }
        }

        n = (n as i32).wrapping_add(r.0).rem_euclid(WHEEL_SIZE as i32) as u32;
    }

    amount
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 6);
    }
}
