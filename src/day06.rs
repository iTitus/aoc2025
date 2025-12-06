use crate::common::parse_split_whitespace;
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub enum Op {
    Add,
    Mul,
}

impl Op {
    pub fn initial(&self) -> u64 {
        match self {
            Op::Add => 0,
            Op::Mul => 1,
        }
    }

    pub fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => return Err(()),
        })
    }
}

#[aoc_generator(day6, part1)]
pub fn input_generator_1(input: &str) -> (Vec<Vec<u64>>, Vec<Op>) {
    let mut lines = vec![];
    let mut last_line = None;
    for l in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        assert!(last_line.is_none());
        if let Ok(line) = parse_split_whitespace(l) {
            lines.push(line);
        } else {
            last_line = Some(parse_split_whitespace(l).unwrap())
        }
    }
    (lines, last_line.unwrap())
}

#[aoc(day6, part1)]
pub fn part1(input: &(Vec<Vec<u64>>, Vec<Op>)) -> u64 {
    let mut result = 0;
    for (i, op) in input.1.iter().enumerate() {
        result += input
            .0
            .iter()
            .map(|l| l[i])
            .fold(op.initial(), |a, e| op.apply(a, e));
    }
    result
}

#[aoc_generator(day6, part2)]
pub fn input_generator_2(input: &str) -> (Vec<Vec<Option<u8>>>, Vec<Op>) {
    let mut lines = vec![];
    let mut last_line = None;
    for l in input.lines().filter(|l| !l.is_empty()) {
        assert!(last_line.is_none());
        if let Ok(ops) = parse_split_whitespace(l) {
            last_line = Some(ops);
        } else {
            lines.push(l.chars().map(|c| c.to_digit(10).map(|n| n as u8)).collect())
        }
    }
    (lines, last_line.unwrap())
}

#[aoc(day6, part2)]
pub fn part2(input: &(Vec<Vec<Option<u8>>>, Vec<Op>)) -> u64 {
    let mut result = 0;
    let mut col = input.0[0].len();
    for op in input.1.iter().rev() {
        let mut problem = op.initial();
        loop {
            col -= 1;
            let mut number_found = false;
            let mut n = 0;
            for l in &input.0 {
                if let Some(d) = l[col] {
                    number_found = true;
                    n = 10 * n + d as u64;
                }
            }

            if number_found {
                problem = op.apply(problem, n);
                if col > 0 {
                    continue;
                }
            }
            break;
        }
        result += problem;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// IntelliJ likes deleting whitespaces, so we concat manually
    const INPUT: &str = concat!(
        "123 328  51 64 \n",
        " 45 64  387 23 \n",
        "  6 98  215 314\n",
        "*   +   *   +  ",
    );

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator_1(INPUT)), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator_2(INPUT)), 3263827);
    }
}
