advent_of_code::solution!(6);

use itertools::Itertools;

type ResT = u64;
type NumT = u64;

fn parse(input: &str) -> (Vec<&str>, Vec<u8>) {
    let mut lines = input.trim_end().lines().collect_vec();
    let ops_s = lines.pop().unwrap();
    let ops = ops_s
        .bytes()
        .filter(|b| !b.is_ascii_whitespace())
        .collect_vec();

    (lines, ops)
}

#[inline]
fn apply_op(op: u8, acc: NumT, x: NumT) -> NumT {
    match op {
        b'+' => acc + x,
        b'*' => acc * x,
        _ => panic!("take estrogen ◕⩊◕"),
    }
}

pub fn part_one(input: &str) -> Option<ResT> {
    let (lines, ops) = parse(input);
    let inits = ops
        .iter()
        .map(|&op| if op == b'+' { 0 } else { 1 })
        .collect_vec();

    Some(
        lines
            .into_iter()
            .fold(inits, |mut accs, line| {
                line.split_whitespace()
                    .map(|t| t.parse::<NumT>().unwrap())
                    .zip(accs.iter_mut())
                    .zip(ops.iter().copied())
                    .for_each(|((x, acc), op)| *acc = apply_op(op, *acc, x));
                accs
            })
            .into_iter()
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<ResT> {
    let (lines, ops) = parse(input);
    let rows = lines.len();
    let cols_max = lines.iter().map(|x| x.len()).max()?;

    let nums = lines
        .iter()
        .map(|line| line.bytes().collect_vec())
        .collect_vec();

    Some(
        (0..(cols_max as NumT))
            .fold(vec![vec![]], |mut acc_out, col| {
                let mut all_blank = true;

                let group = (0..(rows as NumT)).fold(Vec::new(), |mut acc, row| {
                    if nums[row as usize].len() > (col as usize)
                        && nums[row as usize][col as usize] != b' '
                    {
                        all_blank = false;
                        acc.push(nums[row as usize][col as usize] - b'0');
                    }

                    acc
                });

                if all_blank {
                    acc_out.push(Vec::new());
                }

                acc_out.last_mut().unwrap().push(group);
                acc_out
            })
            .into_iter()
            .zip(ops)
            .map(|(group, op)| {
                group
                    .into_iter()
                    .filter(|nums| !nums.is_empty())
                    .map(move |nums| {
                        nums.into_iter()
                            .fold(0, |acc, x| (acc as NumT) * 10 + (x as NumT))
                    })
                    .fold(if op == b'+' { 0 } else { 1 }, |acc, num| {
                        apply_op(op, acc, num)
                    })
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
