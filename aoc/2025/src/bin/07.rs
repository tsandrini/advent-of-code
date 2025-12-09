advent_of_code::solution!(7);

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

type ResT = u64;
type ElemT = u8;
type GridT = Vec<Vec<ElemT>>;

fn parse(input: &str) -> (usize, GridT) {
    let mut lines = input.trim_end().lines();
    let start_x = lines
        .next()
        .unwrap()
        .bytes()
        .position(|b| b == b'S')
        .unwrap();
    let _ = lines.next_back(); // last line is useless
    let grid = lines
        .map(|line| {
            line.bytes()
                .map(|b| match b {
                    b'.' => 0u8,
                    b'^' => 1u8,
                    _ => panic!("**paws at you**"),
                })
                .collect_vec()
        })
        .collect_vec();
    (start_x, grid)
}

pub fn part_one(input: &str) -> Option<ResT> {
    let (start_x, grid) = parse(input);
    let mut init = FxHashSet::default();
    init.insert(start_x);

    Some(
        grid.into_iter()
            .fold((init, 0), |(beams, ctr), row| {
                beams
                    .into_iter()
                    .fold((FxHashSet::default(), ctr), |(mut next, mut ctr), x| {
                        match row[x] {
                            0 => {
                                next.insert(x);
                            }
                            1 => {
                                next.insert(x - 1); // never OOB
                                next.insert(x + 1);
                                ctr += 1;
                            }
                            _ => panic!("**licks you**"),
                        }
                        (next, ctr)
                    })
            })
            .1,
    )
}

pub fn part_two(input: &str) -> Option<ResT> {
    let (start_x, grid) = parse(input);
    let mut init = FxHashMap::default();
    init.insert(start_x, 1);

    Some(
        grid.into_iter()
            .fold((init, 0), |(beams, decisions), row| {
                beams.into_iter().fold(
                    (FxHashMap::default(), decisions),
                    |(mut next, mut decisions), (x, ways)| {
                        match row[x] {
                            0 => {
                                *next.entry(x).or_default() += ways;
                            }
                            1 => {
                                decisions += ways;
                                *next.entry(x - 1).or_default() += ways; // never OOB
                                *next.entry(x + 1).or_default() += ways;
                            }
                            _ => panic!("**sobs at you**"),
                        }
                        (next, decisions)
                    },
                )
            })
            .0
            .values()
            .sum::<ResT>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
