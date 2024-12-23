advent_of_code::solution!(22);

use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::AddAssign;

type SecretNum = i64;
const PRUNE_CONST: SecretNum = 16777216;

fn pack_base10_to_base20(window: &[i64; 4]) -> i64 {
    window.iter().enumerate().fold(0, |acc, (i, &digit)| {
        acc + ((digit + 9) * 20_i64.pow(i as u32))
    })
}

fn update_packed_key(packed: i64, removed: i64, added: i64) -> i64 {
    let updated = (packed - (removed + 9)) / 20;
    updated + ((added + 9) * 20_i64.pow(3))
}

fn mix_n_prune(num: SecretNum, mix_coeff: SecretNum) -> SecretNum {
    (num ^ mix_coeff) % PRUNE_CONST
}

fn evolve_secret_num(num: SecretNum) -> SecretNum {
    let a = mix_n_prune(num, num * 64);
    let b = mix_n_prune(a, a / 32);
    mix_n_prune(b, b * 2048)
}

pub fn part_one(input: &str) -> Option<SecretNum> {
    Some(
        input
            .lines()
            .map(|line| line.parse::<SecretNum>().unwrap())
            .par_bridge()
            .map(|num| (0..2000).fold(num, |acc, _| evolve_secret_num(acc)))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let seqs = input.lines().fold(FxHashMap::default(), |mut acc, line| {
        let mut seen = FxHashSet::default();

        let mut prev_num = line.parse::<SecretNum>().unwrap();
        let mut prev_price = prev_num % 10;
        let mut rolling_window = [0; 4];

        for i in 0..3 {
            let num = evolve_secret_num(prev_num);
            let price = num % 10;
            let diff = price - prev_price;
            rolling_window[i] = diff;
            (prev_num, prev_price) = (num, price);
        }

        // Thanks to https://github.com/rzikm for the idea of using one base20
        // number to represent the rolling window <3 cuts runtime from
        // ~800ms to ~150ms
        let mut packed_key = pack_base10_to_base20(&rolling_window);

        for _ in 3..2000 {
            let num = evolve_secret_num(prev_num);
            let price = num % 10;
            let diff = price - prev_price;

            packed_key = update_packed_key(packed_key, rolling_window[0], diff);
            rolling_window.rotate_left(1);
            rolling_window[3] = diff;

            if seen.insert(packed_key) {
                acc.entry(packed_key).or_insert(0).add_assign(price);
            }

            (prev_num, prev_price) = (num, price);
        }
        acc
    });
    Some(*seqs.values().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            &"1
2
3
2024",
        );
        assert_eq!(result, Some(23));
    }
}
