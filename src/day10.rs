use crate::common::{parse_lines, parse_split};
use aoc_runner_derive::{aoc, aoc_generator};
use good_lp::{
    Expression, ProblemVariables, Solution, SolverModel, constraint, default_solver, variable,
};
use pathfinding::prelude::bfs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Machine {
    lights: Lights,
    buttons: Vec<Buttons>,
    joltages: Joltages,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        let (lights, mut parts) = parts.split_first().ok_or(())?;
        let lights = lights.parse()?;
        let joltages = parts.split_off_last().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Self {
            lights,
            buttons: parts
                .iter()
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| ())?,
            joltages,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Lights {
    state: u32,
}

impl FromStr for Lights {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches(['[', ']']);
        let state: Vec<bool> = s.chars().map(|c| c == '#').collect();
        let mut l = Self::default();
        state
            .iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .for_each(|(i, _)| l.toggle_index(i));
        Ok(l)
    }
}

impl Lights {
    fn toggle_index(&mut self, index: usize) {
        debug_assert!(index < u32::BITS as usize);
        let mask = 1u32 << index;
        if (self.state & mask) != 0 {
            self.state &= !mask;
        } else {
            self.state |= mask;
        }
    }

    pub fn toggle(&mut self, buttons: &Buttons) {
        for &idx in &buttons.indices {
            self.toggle_index(idx as usize);
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Joltages {
    state: Vec<u16>,
}

impl FromStr for Joltages {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches(['{', '}']);
        Ok(Self {
            state: parse_split(s, ',')?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Buttons {
    indices: Vec<u8>,
}

impl FromStr for Buttons {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_matches(['(', ')']);
        Ok(Self {
            indices: parse_split(s, ',')?,
        })
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<Machine> {
    parse_lines(input).unwrap()
}

fn fewest_button_presses_1(m: &Machine) -> usize {
    let path = bfs(
        &Lights::default(),
        |s| {
            let s = s.clone();
            m.buttons.iter().map(move |b| {
                let mut l = s.clone();
                l.toggle(b);
                l
            })
        },
        |s| s == &m.lights,
    )
    .unwrap();
    path.len() - 1
}

#[aoc(day10, part1)]
pub fn part1(input: &[Machine]) -> usize {
    input.iter().map(fewest_button_presses_1).sum()
}

fn fewest_button_presses_2(m: &Machine) -> usize {
    let mut problem = ProblemVariables::new();
    let vars: Vec<_> = m
        .buttons
        .iter()
        .map(|_| problem.add(variable().integer().min(0)))
        .collect();

    let objective = vars.iter().fold(Expression::default(), |a, e| a + e);
    let constraints: Vec<_> = m
        .joltages
        .state
        .iter()
        .enumerate()
        .map(|(idx, &j)| {
            let expr =
                m.buttons
                    .iter()
                    .zip(vars.iter())
                    .fold(Expression::default(), |a, (b, &v)| {
                        a + v * if b.indices.contains(&(idx as u8)) {
                            1
                        } else {
                            0
                        }
                    });
            constraint!(j as i32 == expr)
        })
        .collect();

    let sol = problem
        .minimise(objective.clone())
        .using(default_solver)
        .with_all(constraints)
        .solve()
        .unwrap();
    sol.eval(&objective) as usize
}

#[aoc(day10, part2)]
pub fn part2(input: &[Machine]) -> usize {
    input.iter().map(fewest_button_presses_2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 33);
    }
}
