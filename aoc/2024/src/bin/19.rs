advent_of_code::solution!(19);

use memoize::memoize;
use rustc_hash::FxHashMap;

type StripeT = u8;
type Pattern = Vec<StripeT>;

fn str_to_pattern(s: &str) -> Pattern {
    s.chars()
        .map(|c| match c {
            'w' => 0,
            'u' => 1,
            'b' => 2,
            'r' => 3,
            'g' => 4,
            _ => unreachable!(),
        })
        .collect()
}

fn parse(input: &str) -> (Vec<Pattern>, Vec<Pattern>) {
    let (patterns_s, designs_s) = input.trim().split_once("\n\n").unwrap();
    (
        patterns_s.split(", ").map(|s| str_to_pattern(s)).collect(),
        designs_s.lines().map(|s| str_to_pattern(s)).collect(),
    )
}

#[memoize(Ignore: patterns, CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn is_design_constructable(design: Pattern, patterns: &[Pattern]) -> bool {
    if design.is_empty() {
        return true;
    }

    patterns.iter().any(|pattern| {
        design.len() >= pattern.len()
            && design[..pattern.len()].iter().eq(pattern.iter())
            && is_design_constructable(design[pattern.len()..].to_vec(), patterns)
    })
}

#[memoize(Ignore: patterns, CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn num_combinations_of_design(design: Pattern, patterns: &[Pattern]) -> u64 {
    if design.is_empty() {
        return 1;
    }

    patterns.iter().fold(0, |acc, pattern| {
        if design.len() >= pattern.len() && design[..pattern.len()].iter().eq(pattern.iter()) {
            acc + num_combinations_of_design(design[pattern.len()..].to_vec(), patterns)
        } else {
            acc
        }
    })
}

pub fn part_one(input: &str) -> Option<u16> {
    let (patterns, designs) = parse(input);
    Some(designs.into_iter().fold(0, |acc, design| {
        acc + is_design_constructable(design, &patterns) as u16
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, designs) = parse(input);
    Some(designs.into_iter().fold(0, |acc, design| {
        acc + num_combinations_of_design(design, &patterns)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
