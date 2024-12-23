advent_of_code::solution!(23);

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

fn pack_chars((c1, c2): (char, char)) -> u16 {
    ((c1 as u16) << 8) | (c2 as u16)
}

fn unpack_chars(packed: u16) -> (char, char) {
    (((packed >> 8) as u8 as char), (packed as u8 as char))
}

fn startswith_t(packed: u16) -> bool {
    ((packed >> 8) as u8) == b't'
}

pub fn part_one(input: &str) -> Option<u32> {
    let edges = input
        .lines()
        .map(|l| {
            l.split("-")
                .map(|n| pack_chars(n.chars().collect_tuple().unwrap()))
                .collect_tuple()
                .unwrap()
        })
        .fold(FxHashMap::default(), |mut acc, (a, b)| {
            acc.entry(a).or_insert_with(Vec::new).push(b);
            acc.entry(b).or_insert_with(Vec::new).push(a);
            acc
        });

    let mut count = 0;

    for (node_a, edges_a) in edges.iter() {
        for &node_b in edges_a {
            for &node_c in edges.get(&node_b).unwrap() {
                if node_c == *node_a {
                    continue;
                }

                if edges.get(&node_c).unwrap().contains(node_a)
                    && (startswith_t(*node_a) || startswith_t(node_b) || startswith_t(node_c))
                {
                    count += 1;
                }
            }
        }
    }
    Some(count / 6)
}

fn traverse_edges(
    node: u16,
    required_conns: Vec<u16>,
    edges: &FxHashMap<u16, Vec<u16>>,
    lan_sets: &mut FxHashSet<Vec<u16>>,
) {
    let mut required_conns = required_conns;
    required_conns.sort();
    required_conns.dedup();

    if lan_sets.contains(&required_conns) {
        return;
    }

    lan_sets.insert(required_conns.clone());

    for &next_node in edges.get(&node).unwrap() {
        if required_conns.contains(&next_node)
            || required_conns
                .iter()
                .any(|&conn| !edges.get(&next_node).unwrap().contains(&conn))
        {
            continue;
        }

        let mut next_required_conns = required_conns.clone();
        next_required_conns.push(next_node);
        traverse_edges(next_node, next_required_conns, edges, lan_sets);
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let edges = input
        .lines()
        .map(|l| {
            l.split("-")
                .map(|n| pack_chars(n.chars().collect_tuple().unwrap()))
                .collect_tuple()
                .unwrap()
        })
        .fold(FxHashMap::default(), |mut acc, (a, b)| {
            acc.entry(a).or_insert_with(Vec::new).push(b);
            acc.entry(b).or_insert_with(Vec::new).push(a);
            acc
        });

    let mut lan_sets = FxHashSet::default();
    for &node in edges.keys() {
        traverse_edges(node, vec![node], &edges, &mut lan_sets);
    }

    Some(
        lan_sets
            .iter()
            .max_by_key(|lan_set| lan_set.len())
            .unwrap()
            .iter()
            .map(|&n| {
                let (c1, c2) = unpack_chars(n);
                format!("{}{}", c1, c2)
            })
            .collect::<Vec<_>>()
            .join(","),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            &"ka-co
ta-co
de-co
ta-ka
de-ta
ka-de",
        );
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
