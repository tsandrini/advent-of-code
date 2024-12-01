advent_of_code::solution!(1);

use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = parse(input);

    Some(
        lines
            .iter()
            .map(|(a, _)| a)
            .sorted()
            .zip(lines.iter().map(|(_, b)| b).sorted())
            .map(|(a, b)| if a > b { a - b } else { b - a })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = parse(input);
    let fst = lines.iter().map(|(a, _)| a);
    let snd = lines.iter().map(|(_, b)| b);

    let freq = snd.fold(HashMap::new(), |mut map, val| {
        map.entry(val).and_modify(|f| *f += 1).or_insert(1);
        map
    });

    Some(fst.map(|a| a * (freq.get(a).unwrap_or(&0))).sum())
}

pub fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split_whitespace().collect();
            (split[0].parse().unwrap(), split[1].parse().unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
