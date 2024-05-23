advent_of_code::solution!(3);

use itertools::Itertools;

fn value(c: u8) -> usize {
    match c {
        b'a'..=b'z' => c as usize - b'a' as usize + 1,
        b'A'..=b'Z' => c as usize - b'A' as usize + 27,
        _ => unreachable!(),
    }
}

fn intersection(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().copied().filter(|c| b.contains(c)).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(str::as_bytes)
            .map(|rucksack| {
                intersection(
                    &rucksack[..rucksack.len() / 2],
                    &rucksack[rucksack.len() / 2..],
                )
            })
            .map(|x| value(x[0]) as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(str::as_bytes)
            .tuples()
            .map(|(a, b, c)| intersection(&intersection(a, b), c))
            .map(|x| value(x[0]) as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(157));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(70));
    }
}
