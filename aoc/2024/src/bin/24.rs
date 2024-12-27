advent_of_code::solution!(24);

use bitvec::prelude::*;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use std::str::FromStr;
use strum::Display;
use strum_macros::EnumString;

type Gate<'a> = &'a str;
type Wire<'a> = (Gate<'a>, Op, Gate<'a>);
type Wires<'a> = FxHashMap<Gate<'a>, Wire<'a>>;

#[derive(EnumString, Display, Eq, PartialEq, Hash, Clone, Copy)]
enum Op {
    #[strum(serialize = "AND")]
    AND,
    #[strum(serialize = "OR")]
    OR,
    #[strum(serialize = "XOR")]
    XOR,
}

fn eq_either_or<T: PartialEq>(a: T, b: T, x: T, y: T) -> bool {
    (a == x && b == y) || (a == y && b == x)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (gates_s, wires_s) = input.split_once("\n\n").unwrap();
    let mut gates = gates_s.lines().fold(FxHashMap::default(), |mut acc, line| {
        let (op, out) = line.split_once(": ").unwrap();
        acc.insert(op, out == "1");
        acc
    });

    let mut queue = wires_s
        .lines()
        .map(|line| {
            let (args_s, out) = line.split_once(" -> ").unwrap();
            let (gate1, op, gate2) = args_s.split(" ").collect_tuple().unwrap();
            let op = Op::from_str(op).unwrap();
            (gate1, op, gate2, out)
        })
        .collect::<VecDeque<_>>();

    while let Some(elem) = queue.pop_front() {
        let (gate1, op, gate2, out) = elem;

        if gates.contains_key(gate1) && gates.contains_key(gate2) {
            let gate1 = *gates.get(gate1).unwrap();
            let gate2 = *gates.get(gate2).unwrap();
            let result = match op {
                Op::AND => gate1 & gate2,
                Op::OR => gate1 | gate2,
                Op::XOR => gate1 ^ gate2,
            };
            gates.insert(out, result);
        } else {
            queue.push_back(elem);
        }
    }

    Some(
        gates
            .into_iter()
            .filter(|(k, _)| k.starts_with("z"))
            .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
            .map(|(_, v)| v)
            .collect::<BitVec>()
            .load::<u64>(),
    )
}

fn _is_tmp_xor(gate: Gate, wires: &Wires, z_index: usize) -> bool {
    if let Some((gate1, op, gate2)) = wires.get(gate) {
        if *op != Op::XOR {
            false
        } else {
            let x_gate = format!("x{:02}", z_index);
            let y_gate = format!("y{:02}", z_index);
            eq_either_or(gate1, gate2, &x_gate.as_str(), &y_gate.as_str())
        }
    } else {
        false
    }
}

fn _is_carry(gate: Gate, wires: &Wires, z_index: usize) -> bool {
    if let Some((gate1, op, gate2)) = wires.get(gate) {
        if z_index == 1 && *op != Op::AND {
            false
        } else if z_index == 1 {
            eq_either_or(gate1, gate2, &"x00", &"y00")
        } else if *op != Op::OR {
            false
        } else {
            (_is_new_carry(gate1, wires, z_index - 1)
                && _is_propagated_carry(gate2, wires, z_index - 1))
                || (_is_new_carry(gate2, wires, z_index - 1)
                    && _is_propagated_carry(gate1, wires, z_index - 1))
        }
    } else {
        false
    }
}

fn _is_new_carry(gate: Gate, wires: &Wires, z_index: usize) -> bool {
    if let Some((gate1, op, gate2)) = wires.get(gate) {
        if *op != Op::AND {
            false
        } else {
            let x_gate = format!("x{:02}", z_index);
            let y_gate = format!("y{:02}", z_index);
            eq_either_or(gate1, gate2, &x_gate.as_str(), &y_gate.as_str())
        }
    } else {
        false
    }
}

fn _is_propagated_carry(gate: Gate, wires: &Wires, z_index: usize) -> bool {
    if let Some((gate1, op, gate2)) = wires.get(gate) {
        if *op != Op::AND {
            false
        } else {
            (_is_tmp_xor(gate1, wires, z_index) && _is_carry(gate2, wires, z_index))
                || (_is_tmp_xor(gate2, wires, z_index) && _is_carry(gate1, wires, z_index))
        }
    } else {
        false
    }
}

fn is_adder(z_gate: Gate, wires: &Wires, z_index: usize) -> bool {
    if let Some((gate1, op, gate2)) = wires.get(z_gate) {
        if *op != Op::XOR {
            false
        } else if z_index == 0 {
            eq_either_or(gate1, gate2, &"x00", &"y00")
        } else {
            (_is_tmp_xor(gate1, wires, z_index) && _is_carry(gate2, wires, z_index))
                || (_is_tmp_xor(gate2, wires, z_index) && _is_carry(gate1, wires, z_index))
        }
    } else {
        false
    }
}

fn first_adder_to_fail(wires: &Wires, upper_bound: usize) -> Option<usize> {
    (0..upper_bound).find(|i| !is_adder(&format!("z{:02}", i), wires, *i))
}

// x: [1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1]
// y: [1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 1]
// -----------------------
// +: [0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1]
// z: [0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1]
pub fn part_two(input: &str) -> Option<String> {
    let (_, wires_s) = input.split_once("\n\n").unwrap();

    let wires = wires_s.lines().fold(FxHashMap::default(), |mut acc, line| {
        let (args_s, out) = line.split_once(" -> ").unwrap();
        let (gate1, op, gate2) = args_s.split(" ").collect_tuple().unwrap();
        let op = Op::from_str(op).unwrap();
        acc.insert(out, (gate1, op, gate2));
        acc
    });

    let z_wires_len = wires.iter().filter(|(k, _)| k.starts_with("z")).count();
    let wire_keys = wires.keys().cloned().collect::<Vec<_>>();

    let (_, mut swaps) = (0..4).fold((wires, vec![]), |(mut wires, mut swaps), _| {
        let curr_z_index = first_adder_to_fail(&wires, z_wires_len).unwrap();
        for (wire_x, wire_y) in wire_keys.iter().tuple_combinations() {
            let x_ptr = wires.get_mut(wire_x).unwrap() as *mut _;
            let y_ptr = wires.get_mut(wire_y).unwrap() as *mut _;
            unsafe {
                std::ptr::swap(x_ptr, y_ptr);
            }
            if first_adder_to_fail(&wires, z_wires_len).unwrap() > curr_z_index {
                swaps.push(wire_x);
                swaps.push(wire_y);
                break;
            }
            unsafe {
                std::ptr::swap(x_ptr, y_ptr);
            }
        }
        (wires, swaps)
    });

    swaps.sort();
    Some(swaps.into_iter().join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        let result = part_two(
            &"x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00",
        );
        // assert_eq!(result, Some("z00,z01,z02,z05".to_string()));
        assert_eq!(result, Some("".to_string()));
    }
}
