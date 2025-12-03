use crate::common::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct BatteryBank(pub Vec<u8>);

impl FromStr for BatteryBank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect(),
        ))
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<BatteryBank> {
    parse_lines(input).unwrap()
}

/// returns the first found maximum in the given array, unlike the rust stdlib max functions which return the last
///
/// also includes an optimization to early exit if a global maximum was reached
fn first_max<T: Ord + Copy>(elements: &[T], max_value: T) -> (usize, T) {
    debug_assert!(elements.iter().all(|&x| x <= max_value));
    if elements.is_empty() {
        panic!("empty input");
    }

    if elements[0] == max_value {
        return (0, elements[0]);
    }

    let mut max = 0;
    for (idx, &x) in elements.iter().enumerate().skip(1) {
        if x == max_value {
            return (idx, x);
        }
        if x > elements[max] {
            max = idx;
        }
    }

    (max, elements[max])
}

pub fn max_joltage(bank: &BatteryBank, batteries: u32) -> u64 {
    assert!(batteries as usize <= bank.0.len());

    let mut result = 0;
    let mut start = 0;
    for bat_idx in (0..batteries).rev() {
        let pow10 = 10u64.pow(bat_idx);
        let (idx, max) = first_max(&bank.0[start..(bank.0.len() - bat_idx as usize)], 9);
        start += idx + 1;
        result += pow10 * max as u64;
    }

    result
}

#[aoc(day3, part1)]
pub fn part1(input: &[BatteryBank]) -> u64 {
    input.iter().map(|b| max_joltage(b, 2)).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[BatteryBank]) -> u64 {
    input.iter().map(|b| max_joltage(b, 12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 3121910778619);
    }
}
