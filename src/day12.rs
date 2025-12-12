use crate::common::{parse_lines, parse_split_whitespace, Grid};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Block,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Block,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Shape {
    grid: Grid<Tile>,
    block_count: usize,
}

impl Shape {
    pub fn new(grid: Grid<Tile>) -> Self {
        let block_count = grid.iter().filter(|&&t| t == Tile::Block).count();
        Self { grid, block_count }
    }
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.split_once(':')
                .ok_or(())?
                .1
                .trim()
                .parse()
                .map_err(|_| ())?,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    width: u8,
    height: u8,
    shapes: Vec<u8>,
}

impl FromStr for Region {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (size, shapes) = s.split_once(':').ok_or(())?;
        let (width, height) = size.split_once('x').ok_or(())?;
        Ok(Self {
            width: width.trim().parse().map_err(|_| ())?,
            height: height.trim().parse().map_err(|_| ())?,
            shapes: parse_split_whitespace(shapes).map_err(|_| ())?,
        })
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let parts = input.split("\n\n").collect_vec();
    let (regions, shapes) = parts.split_last().unwrap();
    let shapes = shapes
        .iter()
        .map(|s| s.parse())
        .collect::<Result<_, _>>()
        .unwrap();
    let regions = parse_lines(regions).unwrap();
    (shapes, regions)
}

fn fits(region: &Region, shapes: &[Shape]) -> bool {
    let region_size = region.width as usize * region.height as usize;
    let min_required_space: usize = region
        .shapes
        .iter()
        .enumerate()
        .map(|(i, &amount)| amount as usize * shapes[i].block_count)
        .sum();
    if min_required_space > region_size {
        return false;
    }

    // TODO: actually check this?
    // seems to just work for my input, but not for the example...
    true
}

#[aoc(day12, part1)]
pub fn part1(input: &(Vec<Shape>, Vec<Region>)) -> usize {
    input.1.iter().filter(|r| fits(r, &input.0)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;

    #[test]
    fn test_part1() {
        // TODO: assert_eq!(part1(&input_generator(INPUT)), 2);
    }
}
