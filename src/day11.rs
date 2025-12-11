use crate::common::{parse_lines, parse_split_whitespace};
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::{FxHashMap, FxHashSet};
use std::str::FromStr;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> FxHashMap<String, FxHashSet<String>> {
    struct I(String, Vec<String>);

    impl FromStr for I {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (device, wires) = s.split_once(':').ok_or(())?;
            Ok(I(
                device.trim().into(),
                parse_split_whitespace(wires).map_err(|_| ())?,
            ))
        }
    }

    let v: Vec<I> = parse_lines(input).unwrap();
    v.into_iter()
        .map(|i| (i.0, FxHashSet::from_iter(i.1)))
        .collect()
}

fn count_paths_from_to(
    input: &FxHashMap<String, FxHashSet<String>>,
    from: &str,
    to: &str,
) -> usize {
    pathfinding::prelude::count_paths(
        from,
        |device| input.get(*device).into_iter().flatten().map(|s| s.as_str()),
        |device| *device == to,
    )
}

#[aoc(day11, part1)]
pub fn part1(input: &FxHashMap<String, FxHashSet<String>>) -> usize {
    count_paths_from_to(input, "you", "out")
}

#[aoc(day11, part2)]
pub fn part2(input: &FxHashMap<String, FxHashSet<String>>) -> usize {
    let input_without_fft: FxHashMap<String, FxHashSet<String>> = input
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.iter().filter(|s| *s != "fft").cloned().collect(),
            )
        })
        .collect();
    let input_without_dac = input
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.iter().filter(|s| *s != "dac").cloned().collect(),
            )
        })
        .collect();
    let input_without_dac_and_fft = input
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.iter()
                    .filter(|s| *s != "dac" && *s != "fft")
                    .cloned()
                    .collect(),
            )
        })
        .collect();
    let a1 = count_paths_from_to(&input_without_fft, "svr", "dac");
    let a2 = count_paths_from_to(&input_without_dac, "dac", "fft");
    let a3 = count_paths_from_to(&input_without_dac_and_fft, "fft", "out");
    let b1 = count_paths_from_to(&input_without_dac, "svr", "fft");
    let b2 = count_paths_from_to(&input_without_fft, "fft", "dac");
    let b3 = count_paths_from_to(&input_without_dac_and_fft, "dac", "out");
    a1 * a2 * a3 + b1 * b2 * b3
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    const INPUT2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT2)), 2);
    }
}
