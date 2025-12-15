advent_of_code::solution!(11);

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

type ResT = u64;
type NumT = u64;

fn parse(input: &str) -> FxHashMap<&str, Vec<&str>> {
    input
        .trim()
        .lines()
        .fold(FxHashMap::default(), |mut acc, line| {
            let (key, vals) = line.split_once(':').unwrap();
            acc.entry(key)
                .or_insert_with(Vec::new)
                .extend(vals.split_whitespace().collect_vec());

            acc
        })
}

fn solve<'a>(
    input: &'a str,
    start: &'a str,
    end: &'a str,
    mid1: &'a str,
    mid2: &'a str,
) -> FxHashMap<&'a str, [NumT; 4]> {
    #[inline]
    fn bit(node: &str, mid1: &str, mid2: &str) -> usize {
        (node == mid1) as usize | (((node == mid2) as usize) << 1)
    }

    let graph = parse(input);
    let mut paths = FxHashMap::default();
    let mut seen = FxHashSet::default();
    let mut q = VecDeque::default();

    q.push_back((start, false));

    while let Some((node, expanded)) = q.pop_back() {
        match expanded {
            true => {
                let idx = bit(node, mid1, mid2);
                let node_counters = if node == end {
                    let mut a = [0; 4];
                    a[idx] = 1;
                    a
                } else {
                    graph
                        .get(node)
                        .into_iter()
                        .flatten()
                        .fold([0; 4], |mut acc, &u| {
                            let children = *paths.get(u).unwrap_or(&[0; 4]);
                            (0..4).for_each(|m| acc[m | idx] += children[m]);
                            acc
                        })
                };

                paths.insert(node, node_counters);
                seen.insert(node);
                continue;
            }
            false => {
                if seen.contains(node) {
                    continue;
                }

                q.push_back((node, true));

                if node != end
                    && let Some(neigh) = graph.get(node)
                {
                    for &u in neigh.iter() {
                        if !seen.contains(u) {
                            q.push_back((u, false));
                        }
                    }
                }
            }
        }
    }

    paths
}

pub fn part_one(input: &str) -> Option<ResT> {
    let paths = solve(input, "you", "out", "", "");
    Some(paths.get("you").unwrap().iter().sum::<NumT>() as ResT)
}

pub fn part_two(input: &str) -> Option<ResT> {
    let paths = solve(input, "svr", "out", "dac", "fft");
    Some(paths.get("svr").unwrap()[3] as ResT) // 11 -> both mid1 && mid2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let part2_example_input = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let result = part_two(part2_example_input);
        assert_eq!(result, Some(2));
    }
}
