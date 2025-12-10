advent_of_code::solution!(9);

use itertools::Itertools;
use rustc_hash::FxHashMap;

type ResT = u64;
type NumT = i64;
type P = (NumT, NumT);

fn parse(input: &str) -> Vec<P> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<NumT>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<ResT> {
    Some(
        parse(input)
            .into_iter()
            .combinations(2)
            .map(|x| {
                let ((x1, y1), (x2, y2)) = (x[0], x[1]);
                x2.abs_diff(x1 + 1) * y2.abs_diff(y1 + 1)
            })
            .max()
            .unwrap() as ResT,
    )
}

pub fn part_two(input: &str) -> Option<ResT> {
    let pts = parse(input);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
