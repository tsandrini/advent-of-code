advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(
        re.captures_iter(input)
            .map(|c| c.extract())
            .fold(0, |acc, (_, [a, b])| {
                acc + (a.parse::<u32>().unwrap_or(0) * b.parse::<u32>().unwrap_or(0))
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)").unwrap();

    Some(
        re.captures_iter(input)
            .fold((0, 1), |(acc, switch), cap| {
                if let Some(mul_a) = cap.get(1) {
                    let a = mul_a.as_str().parse::<u32>().unwrap_or(0);
                    let b = cap.get(2).unwrap().as_str().parse::<u32>().unwrap_or(0);
                    (acc + switch * a * b, switch)
                } else if cap.get(0).map_or(false, |m| m.as_str() == "do()") {
                    (acc, 1)
                } else if cap.get(0).map_or(false, |m| m.as_str() == "don't()") {
                    (acc, 0)
                } else {
                    (acc, switch)
                }
            })
            .0 as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result =
            part_one(&"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two(&"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
