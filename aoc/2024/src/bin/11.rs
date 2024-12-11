advent_of_code::solution!(11);

type StoneT = u64;
type CyclesT = u8;
type Cache = rustc_hash::FxHashMap<(StoneT, CyclesT), usize>;

fn compute_stones_dp(stone: StoneT, cycles_left: CyclesT, cache: &mut Cache) -> usize {
    if let Some(&cached_result) = cache.get(&(stone, cycles_left)) {
        return cached_result;
    }

    if cycles_left == 0 {
        return 1;
    }

    let result = if stone == 0 {
        compute_stones_dp(1, cycles_left - 1, cache)
    } else {
        let digits = {
            let mut temp = stone;
            let mut digits = Vec::new();
            while temp > 0 {
                digits.push(temp % 10);
                temp /= 10;
            }
            digits.reverse();
            digits
        };

        if digits.len() % 2 == 0 {
            let mid = digits.len() / 2;
            let left = digits[0..mid].iter().fold(0, |acc, &d| acc * 10 + d);
            let right = digits[mid..].iter().fold(0, |acc, &d| acc * 10 + d);

            let left_count = compute_stones_dp(left, cycles_left - 1, cache);
            let right_count = compute_stones_dp(right, cycles_left - 1, cache);

            left_count + right_count
        } else {
            compute_stones_dp(stone * 2024, cycles_left - 1, cache)
        }
    };

    cache.insert((stone, cycles_left), result);

    result
}

fn solver(input: &str, num_cycles: CyclesT) -> usize {
    let mut cache = Cache::default();

    input
        .split_whitespace()
        .map(|s| compute_stones_dp(s.parse::<StoneT>().unwrap(), num_cycles, &mut cache))
        .sum::<usize>()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solver(input, 25))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solver(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
