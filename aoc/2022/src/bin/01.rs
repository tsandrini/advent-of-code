advent_of_code::solution!(1);

use itertools::Itertools;

pub fn parse_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|items| items.lines().map(|item| item.parse::<u32>().unwrap()).sum())
        .sorted()
        .rev()
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_input(input)[0])
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_input(input)[0..3].iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
