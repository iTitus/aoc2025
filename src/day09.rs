use crate::common::{ParseVecError, Vec2i, parse_lines, parse_vec};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec2i> {
    struct I(Vec2i);

    impl FromStr for I {
        type Err = ParseVecError<ParseIntError>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(I(parse_vec(s)?))
        }
    }

    let v: Vec<I> = parse_lines(input).unwrap();
    v.into_iter().map(|i| i.0).collect()
}

fn rectangle_area(a: Vec2i, b: Vec2i) -> u64 {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

#[aoc(day9, part1)]
pub fn part1(input: &[Vec2i]) -> u64 {
    input
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| rectangle_area(a, b))
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Vec2i]) -> u64 {
    let mut max_area = 0;
    'outer: for (&a, &b) in input.iter().tuple_combinations() {
        let min_x = a.x.min(b.x);
        let max_x = a.x.max(b.x);
        let min_y = a.y.min(b.y);
        let max_y = a.y.max(b.y);

        // optimization: calculate area of every rectangle
        // and prune it directly if it cannot be bigger than our current max
        let area = rectangle_area(a, b);
        if area < max_area {
            continue 'outer;
        }

        for (&start, &end) in input.iter().circular_tuple_windows() {
            let start_x = start.x.min(end.x);
            let end_x = start.x.max(end.x);
            let start_y = start.y.min(end.y);
            let end_y = start.y.max(end.y);

            // check if the current edge [start-end] of the polygon intersects with the inside of the current rectangle
            // we have to allow edges that lie on the border of the rectangle
            // this is trivial, because the rectangle and the edge are axis-aligned,
            // and we know that the edges of the rectangle are already inside the polygon by construction
            // so we just care about the insides of the rectangle, every 2x2 and smaller rectangle is already valid
            if end_x > min_x && start_x < max_x && end_y > min_y && start_y < max_y {
                continue 'outer;
            }
        }

        max_area = area;
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 24);
    }
}
