advent_of_code::solution!(9);

use itertools::Itertools;

type ResT = u64;
type NumT = i64;
type P = (NumT, NumT);
type Edge = (P, P);
type Rect = (P, P); // ((x1, y1), (x2, y2))

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
            .map(|x| (x[0], x[1]))
            .map(|((x1, y1), (x2, y2))| x2.abs_diff(x1 + 1) * y2.abs_diff(y1 + 1))
            .max()
            .unwrap() as ResT,
    )
}

fn intersects(((rxmin, rxmax), (rymin, rymax)): Rect, edges: &[Edge]) -> bool {
    edges.iter().any(|&((exmin, exmax), (eymin, eymax))| {
        (rxmin < exmax && rxmax > exmin) && (rymin < eymax && rymax > eymin)
    })
}

pub fn part_two(input: &str) -> Option<ResT> {
    let pts = parse(input);
    let edges = pts
        .clone()
        .into_iter()
        .circular_tuple_windows()
        .map(|((x1, y1), (x2, y2))| {
            (
                (if x1 <= x2 { (x1, x2) } else { (x2, x1) }),
                (if y1 <= y2 { (y1, y2) } else { (y2, y1) }),
            )
        })
        .collect_vec();

    Some(
        pts.into_iter()
            .combinations(2)
            .map(|x| (x[0], x[1]))
            .map(|((x1, y1), (x2, y2))| {
                (
                    (if x1 <= x2 { (x1, x2) } else { (x2, x1) }),
                    (if y1 <= y2 { (y1, y2) } else { (y2, y1) }),
                )
            })
            .filter(|&rect| !intersects(rect, &edges))
            .map(|((x1, x2), (y1, y2))| (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1))
            .max()
            .unwrap() as ResT,
    )
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
