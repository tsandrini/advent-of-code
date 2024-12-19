advent_of_code::solution!(19);

use priority_queue::PriorityQueue;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

type StripeT = u8;
type Pattern = Vec<StripeT>;

fn str_to_pattern(s: &str) -> Pattern {
    s.chars()
        .map(|c| match c {
            'w' => 0,
            'u' => 1,
            'b' => 2,
            'r' => 3,
            'g' => 4,
            _ => unreachable!(),
        })
        .collect()
}

fn parse(input: &str) -> (Vec<Pattern>, Vec<Pattern>) {
    let (patterns_s, designs_s) = input.trim().split_once("\n\n").unwrap();
    (
        patterns_s.split(", ").map(|s| str_to_pattern(s)).collect(),
        designs_s.lines().map(|s| str_to_pattern(s)).collect(),
    )
}

pub fn part_one(input: &str) -> Option<u16> {
    let (mut patterns, designs) = parse(input);
    patterns.sort_by_key(|p| p.len());

    Some(
        designs
            .par_iter()
            .filter(|design| {
                let mut pq: PriorityQueue<Pattern, usize> = PriorityQueue::new();
                pq.push(vec![], 0);

                while let Some((curr_pattern, curr_len)) = pq.pop() {
                    if curr_len == design.len() {
                        return true;
                    }

                    for candidate in patterns.iter() {
                        if curr_len + candidate.len() > design.len() {
                            break;
                        }

                        if design[curr_len..(curr_len + candidate.len())]
                            .iter()
                            .eq(candidate.iter())
                        {
                            let mut new_pattern = curr_pattern.clone();
                            let new_pattern_len = curr_len + candidate.len();
                            new_pattern.extend(candidate.iter());
                            pq.push(new_pattern, new_pattern_len);
                        }
                    }
                }

                false
            })
            .count() as u16,
    )
}

fn num_combinations_of_design<'a>(
    design: &'a [StripeT],
    patterns: &[Pattern],
    cache: &mut FxHashMap<&'a [StripeT], u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(&count) = cache.get(design) {
        return count;
    }

    let mut count = 0;
    for pattern in patterns {
        if design.len() >= pattern.len() && design[..pattern.len()].iter().eq(pattern.iter()) {
            count += num_combinations_of_design(&design[pattern.len()..], patterns, cache);
        }
    }

    cache.insert(design, count);
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut patterns, designs) = parse(input);
    let mut cache = FxHashMap::default();
    patterns.sort_by_key(|p| p.len());

    Some(designs.iter().fold(0, |acc, design| {
        acc + num_combinations_of_design(design, &patterns, &mut cache)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
