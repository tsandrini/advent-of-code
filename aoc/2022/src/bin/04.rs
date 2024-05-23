advent_of_code::solution!(4);

use itertools::Itertools;

fn is_subinterval(interval: (u32, u32), subinterval: (u32, u32)) -> bool {
    interval.0 <= subinterval.0 && interval.1 >= subinterval.1
}

fn is_overlapping(a: (u32, u32), b: (u32, u32)) -> bool {
    (a.0 <= b.0 && a.1 >= b.0) || (a.0 <= b.1 && a.1 >= b.1)
}

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let (a, b) = line
        .split(',')
        .map(|s| {
            let (start, end) = s
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .tuples::<(_, _)>()
                .next()
                .unwrap();
            (start, end)
        })
        .tuples::<(_, _)>()
        .next()
        .unwrap();
    (a, b)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (a, b) = { parse_line(line) };
                is_subinterval(a, b) || is_subinterval(b, a)
            })
            .map(|x| if x { 1 } else { 0 })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let (a, b) = { parse_line(line) };
                is_overlapping(a, b) || is_overlapping(b, a)
            })
            .map(|x| if x { 1 } else { 0 })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
