advent_of_code::solution!(8);

use itertools::Itertools;
use petgraph::unionfind::UnionFind;

type ResT = usize;
type NumT = i64;
type P = [NumT; 3];

#[inline(always)]
fn dist2(a: &P, b: &P) -> u64 {
    let dx = a[0].abs_diff(b[0]);
    let dy = a[1].abs_diff(b[1]);
    let dz = a[2].abs_diff(b[2]);
    dx * dx + dy * dy + dz * dz
}

fn parse(input: &str) -> Vec<P> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<NumT>().unwrap())
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<ResT> {
    let pts = parse(input);
    let pts_ref = &pts;
    let n = pts_ref.len();
    let mut uf = UnionFind::new(n);
    let k = 10; // NOTE: 1000 in real input

    (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (dist2(&pts_ref[i], &pts_ref[j]), i, j)))
        .sorted_unstable_by_key(|&(dist, _, _)| dist)
        .take(k)
        .for_each(|(_, i, j)| {
            if !uf.equiv(i, j) {
                uf.union(i, j);
            }
        });

    Some(
        (0..n)
            .fold(vec![0; n], |mut acc, i| {
                acc[uf.find(i)] += 1;
                acc
            })
            .into_iter()
            .filter(|&s| s != 0)
            .k_largest(3)
            .product(),
    )
}

pub fn part_two(input: &str) -> Option<ResT> {
    let pts = parse(input);
    let pts_ref = &pts;
    let n = pts_ref.len();
    let mut uf = UnionFind::new(n);
    let mut comps = n;

    Some(
        (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (dist2(&pts_ref[i], &pts_ref[j]), i, j)))
            .sorted_unstable_by_key(|&(dist, _, _)| dist)
            .find_map(|(_d, i, j)| {
                if uf.equiv(i, j) {
                    return None;
                }
                uf.union(i, j);
                comps -= 1;
                (comps == 1).then_some([i, j])
            })
            .unwrap()
            .iter()
            .map(|&idx| pts_ref[idx][0] as ResT)
            .product(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
