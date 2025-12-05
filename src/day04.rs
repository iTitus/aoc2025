use crate::common::{Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Paper,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Empty,
            '@' => Tile::Paper,
            _ => return Err(()),
        })
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    input.parse().unwrap()
}

fn can_access(grid: &Grid<Tile>, pos: Vec2i) -> bool {
    const NEIGHBORS: [Vec2i; 8] = [
        Vec2i::new(-1, -1),
        Vec2i::new(0, -1),
        Vec2i::new(1, -1),
        Vec2i::new(-1, 0),
        Vec2i::new(1, 0),
        Vec2i::new(-1, 1),
        Vec2i::new(0, 1),
        Vec2i::new(1, 1),
    ];

    let paper_neighbors = NEIGHBORS
        .iter()
        .map(|&dir| pos + dir)
        .filter(|pos| grid.in_bounds(pos))
        .filter(|&pos| grid[pos] == Tile::Paper)
        .count();
    paper_neighbors < 4
}

#[aoc(day4, part1)]
pub fn part1(grid: &Grid<Tile>) -> usize {
    let mut n = 0;
    for (pos, tile) in grid.pos_iter() {
        if *tile == Tile::Paper && can_access(grid, pos) {
            n += 1;
        }
    }

    n
}

#[aoc(day4, part2)]
pub fn part2(grid: &Grid<Tile>) -> u64 {
    let mut grid = grid.clone();
    let mut n = 0;
    loop {
        let mut did_something = false;
        let mut next_grid = grid.clone();
        for (pos, tile) in grid.pos_iter() {
            if *tile == Tile::Paper && can_access(&grid, pos) {
                next_grid[pos] = Tile::Empty;
                n += 1;
                did_something = true;
            }
        }

        if !did_something {
            break;
        }
        grid = next_grid;
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 43);
    }
}
