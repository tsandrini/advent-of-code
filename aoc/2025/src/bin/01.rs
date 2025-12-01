advent_of_code::solution!(1);

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| {
            let (dir, n) = line.split_at(1);
            let dir = match dir {
                "R" => 1i64,
                "L" => -1i64,
                _ => 1i64,
            };
            let num = n.parse::<i64>().unwrap();
            (dir, num)
        })
        .collect::<Vec<_>>()
}

fn zero_hits(acc: i64, dir: i64, rot: i64) -> u64 {
    let acc = acc.rem_euclid(100) as u64;
    let rot = rot as u64;

    let first = match dir {
        1 => (100 - acc) % 100,
        -1 => acc,
        _ => unreachable!(),
    };
    let first = if first == 0 { 100 } else { first };

    rot.saturating_sub(first).div_euclid(100) + (rot >= first) as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .iter()
            .scan(50, |acc, (dir, rot)| {
                *acc = (*acc + dir * rot).rem_euclid(100);
                Some(*acc)
            })
            .filter(|&state| state == 0)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .iter()
            .fold((50, 0), |(acc, hits), &(dir, rot)| {
                let next = (acc + dir * rot).rem_euclid(100);
                (next, hits + zero_hits(acc, dir, rot))
            })
            .1,
    )
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
        assert_eq!(result, Some(6));
    }
}
