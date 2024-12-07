advent_of_code::solution!(7);

use itertools::Itertools;
use rayon::prelude::*;

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (test_vals, nums) = line.split_once(":").unwrap();
            (
                test_vals.parse().unwrap(),
                nums.trim()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec()
}

fn concat(a: u64, b: u64) -> u64 {
    let mut b_temp = b;
    let mut multiplier = 1;
    while b_temp > 0 {
        multiplier *= 10;
        b_temp /= 10;
    }
    a * multiplier + b
}

fn check_base_arithmetic(nums: &[u64], target: u64) -> bool {
    let ranges = vec![[0u8, 1u8]; nums.len()];
    ranges.into_iter().multi_cartesian_product().any(|indices| {
        let sum = nums.iter().zip(indices).fold(0, |acc, (n, i)| match i {
            0 => acc + n,
            1 => acc * n,
            _ => unreachable!(),
        });
        sum == target
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .par_iter()
            .filter(|(test_val, nums)| check_base_arithmetic(nums, *test_val))
            .map(|(test_val, _)| test_val)
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let eqs = parse(input);
    let (valid, dubious): (Vec<_>, Vec<_>) = eqs
        .par_iter()
        .partition(|(test_val, nums)| check_base_arithmetic(nums, *test_val));

    Some(
        valid.par_iter().map(|(test_val, _)| test_val).sum::<u64>()
            + dubious
                .par_iter()
                .filter(|(test_val, nums)| {
                    let ranges = vec![[0u8, 1u8, 2u8]; nums.len()];
                    ranges.into_iter().multi_cartesian_product().any(|indices| {
                        let sum = nums.iter().zip(indices).fold(0, |acc, (n, i)| match i {
                            0 => acc + n,
                            1 => acc * n,
                            2 => concat(acc, *n),
                            _ => unreachable!(),
                        });
                        sum == *test_val
                    })
                })
                .map(|(test_val, _)| test_val)
                .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
