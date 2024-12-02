advent_of_code::solution!(2);

fn signum(x: i32) -> i32 {
    if x > 0 {
        1
    } else if x < 0 {
        -1
    } else {
        0
    }
}
fn gen_subvectors<T: Clone>(input: Vec<T>) -> Vec<Vec<T>> {
    (0..input.len())
        .map(|i| {
            input[..i]
                .iter()
                .cloned()
                .chain(input[i + 1..].iter().cloned())
                .collect()
        })
        .collect()
}

fn validate_report(report: &Vec<i32>) -> bool {
    let diffs = report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>();

    diffs.iter().all(|&x| x.abs() <= 3 && x != 0)
        && diffs.iter().map(|&x| signum(x)).sum::<i32>().abs() == (diffs.len() as i32)
}

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        parse(input)
            .iter()
            .filter(|report| validate_report(&report))
            .count() as i32,
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let reports = parse(input);
    let (safe, dubious): (Vec<_>, Vec<_>) =
        reports.iter().partition(|report| validate_report(&report));

    Some(
        (safe.len()
            + dubious
                .iter()
                .filter(|report| {
                    gen_subvectors(report.to_vec())
                        .iter()
                        .any(|report| validate_report(&report))
                })
                .count()) as i32,
    )
}

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
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
