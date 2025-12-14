advent_of_code::solution!(10);

use good_lp::solvers::microlp::microlp;
use good_lp::{Expression, Solution, SolverModel, constraint, variables};
use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

type ResT = u64;
type BitmaskT = u64;
type JoltageT = u32;

type LightConfig = BitmaskT;
type ButtonConfig = Vec<BitmaskT>;
type JoltageConfig = Vec<JoltageT>;

type MachineConfig = (LightConfig, ButtonConfig, JoltageConfig);

#[inline]
fn set_bit(mut acc: BitmaskT, i: usize) -> BitmaskT {
    acc |= (1 as BitmaskT) << i;
    acc
}

fn parse(input: &str) -> impl ParallelIterator<Item = MachineConfig> {
    input.trim().par_lines().map(|line| {
        let mut parts = line.split_whitespace();
        let lights = parts
            .next()
            .unwrap()
            .bytes()
            .skip(1)
            .dropping_back(1)
            .enumerate()
            .fold(
                0,
                |acc, (i, b)| if b == b'#' { set_bit(acc, i) } else { acc },
            );
        let joltage = parts
            .next_back()
            .unwrap()
            .trim_matches(|c| matches!(c, '{' | '}'))
            .split(',')
            .map(|s| s.parse::<JoltageT>().unwrap())
            .collect_vec();
        let buttons = parts
            .map(|part| {
                part.trim_matches(|c| matches!(c, '(' | ')'))
                    .split(',')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>().unwrap())
                    .fold(0, set_bit)
            })
            .collect_vec();

        (lights, buttons, joltage)
    })
}

pub fn part_one(input: &str) -> Option<ResT> {
    Some(
        parse(input)
            .map(|(lights_target, btns, _)| {
                let mut seen = FxHashSet::default();
                let mut q = VecDeque::new();

                q.push_back((0, 0));
                seen.insert(0);

                while let Some((curr_lights, presses)) = q.pop_front() {
                    if curr_lights == lights_target {
                        return presses;
                    }

                    btns.iter().for_each(|btn| {
                        let next_lights = curr_lights ^ btn;
                        if seen.insert(next_lights) {
                            q.push_back((next_lights, presses + 1));
                        }
                    });
                }

                0
            })
            .sum::<ResT>(),
    )
}

pub fn part_two(input: &str) -> Option<ResT> {
    Some(
        parse(input)
            .map(|(_, btns, jolt_target)| {
                variables! {
                    vars:
                        0 <= coeffs[btns.len()] (integer);
                };

                let mut model = vars
                    .minimise(coeffs.iter().sum::<Expression>())
                    .using(microlp);

                for (i, &t) in jolt_target.iter().enumerate() {
                    let expr = coeffs
                        .iter()
                        .zip(btns.iter())
                        .filter(|&(_, mask)| ((mask >> i) & 1) != 0)
                        .map(|(&xj, _)| xj)
                        .sum::<Expression>();

                    model = model.with(constraint!(expr == t as JoltageT));
                }

                model
                    .solve()
                    .unwrap()
                    .eval(coeffs.iter().sum::<Expression>())
                    .round() as JoltageT
            })
            .sum::<JoltageT>() as ResT,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
