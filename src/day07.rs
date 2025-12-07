use crate::common::{Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Start,
    Splitter,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            'S' => Self::Start,
            '^' => Self::Splitter,
            _ => return Err(()),
        })
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    input.parse().unwrap()
}

#[aoc(day7, part1)]
pub fn part1(grid: &Grid<Tile>) -> usize {
    let (start, _) = grid
        .pos_iter()
        .find(|(_, tile)| **tile == Tile::Start)
        .unwrap();

    let mut visited_splitters = FxHashSet::default();
    let mut q: VecDeque<_> = [start].into();
    while let Some(mut pos) = q.pop_front() {
        while grid.in_bounds(&pos) && grid[pos] != Tile::Splitter {
            pos += Vec2i::new(0, 1);
        }

        if grid.in_bounds(&pos) && visited_splitters.insert(pos) {
            let l = pos + Vec2i::new(-1, 0);
            if grid.in_bounds(&l) {
                q.push_back(l);
            }
            let r = pos + Vec2i::new(1, 0);
            if grid.in_bounds(&r) {
                q.push_back(r);
            }
        }
    }

    visited_splitters.len()
}

#[aoc(day7, part2)]
pub fn part2(grid: &Grid<Tile>) -> usize {
    let (start, _) = grid
        .pos_iter()
        .find(|(_, tile)| **tile == Tile::Start)
        .unwrap();

    let mut g: Grid<usize> = Grid::new_from_element(grid.size_x, grid.size_y, 1);
    let sx = grid.size_x as i64;
    let sy = grid.size_y as i64;

    for y in (0..sy - 1).rev() {
        for x in 0..sx {
            let p = Vec2i::new(x, y);
            if grid[p] == Tile::Splitter {
                let l = p + Vec2i::new(-1, 1);
                let r = p + Vec2i::new(1, 1);
                g[p] =
                    if g.in_bounds(&l) { g[l] } else { 0 } + if g.in_bounds(&r) { g[r] } else { 0 };
            } else {
                g[p] = g[p + Vec2i::new(0, 1)];
            }
        }
    }

    g[start]
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 40);
    }
}
