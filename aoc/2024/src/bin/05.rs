advent_of_code::solution!(5);

use itertools::Itertools;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);

    Some(
        updates
            .iter()
            .filter(|update| {
                update.iter().enumerate().all(|(i, entry)| {
                    if i == 0 || !rules.contains_key(entry) {
                        true
                    } else {
                        let entry_rules = rules.get(entry).unwrap();
                        !update[..i].iter().any(|prev| entry_rules.contains(prev))
                    }
                })
            })
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);

    let mut incorrect_updates = updates
        .clone()
        .into_iter()
        .filter(|update| {
            update.iter().enumerate().any(|(i, entry)| {
                if i == 0 || !rules.contains_key(entry) {
                    false
                } else {
                    let entry_rules = rules.get(entry).unwrap();
                    update[..i].iter().any(|prev| entry_rules.contains(prev))
                }
            })
        })
        .collect_vec();

    incorrect_updates.iter_mut().for_each(|update| {
        let mut i = 0;
        while i < update.len() {
            if !rules.contains_key(&update[i]) {
                i += 1;
                continue;
            }
            let entry_rules = rules.get(&update[i]).unwrap();
            let mut j = i + 1;
            while j < update.len() {
                if entry_rules.contains(&update[j]) {
                    update.swap(i, j);
                    break;
                }
                j += 1;
            }
            if j == update.len() {
                i += 1;
            }
        }
    });

    Some(
        incorrect_updates
            .iter()
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();
    (
        rules_str
            .lines()
            .map(|rule| {
                rule.split("|")
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .fold(HashMap::new(), |mut map, (k, v)| {
                map.entry(k).and_modify(|x| x.push(v)).or_insert(vec![v]);
                map
            }),
        updates_str
            .lines()
            .map(|update| update.split(",").map(|n| n.parse().unwrap()).collect())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
