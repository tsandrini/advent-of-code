advent_of_code::solution!(5);

use itertools::Itertools;

type ElemT = u64;

pub struct IntervalSet {
    merged: Vec<(ElemT, ElemT)>,
}

impl IntervalSet {
    pub fn from_input(input: &str) -> Self {
        let merged = {
            let mut v = input
                .lines()
                .map(|line| {
                    line.split("-")
                        .map(|s| s.parse::<ElemT>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect::<Vec<_>>();

            v.sort_unstable_by_key(|&(lo, _)| lo);

            v.into_iter().fold(Vec::new(), |mut acc, (lo, hi)| {
                match acc.last_mut() {
                    None => acc.push((lo, hi)),
                    Some((_, prev_hi)) => {
                        if lo <= *prev_hi + 1 {
                            *prev_hi = (*prev_hi).max(hi);
                        } else {
                            acc.push((lo, hi));
                        }
                    }
                }
                acc
            })
        };

        Self { merged }
    }

    #[inline]
    pub fn contains(&self, x: ElemT) -> bool {
        let i = self.merged.partition_point(|&(lo, _)| lo <= x);
        i != 0 && x <= self.merged[i - 1].1
    }

    pub fn cardinality(&self) -> ElemT {
        self.merged.iter().map(|&(lo, hi)| hi - lo + 1).sum()
    }
}

pub fn part_one(input: &str) -> Option<ElemT> {
    let (ranges, xs) = input.trim().split_once("\n\n").unwrap();
    let set = IntervalSet::from_input(ranges);

    Some(
        xs.lines()
            .map(|s| s.parse::<ElemT>().unwrap())
            .filter(|&x| set.contains(x))
            .count() as ElemT,
    )
}

pub fn part_two(input: &str) -> Option<ElemT> {
    let (ranges, _) = input.trim().split_once("\n\n").unwrap();
    let set = IntervalSet::from_input(ranges);

    Some(set.cardinality() as ElemT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
