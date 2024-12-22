advent_of_code::solution!(21);

use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use std::str::FromStr;
use strum_macros::EnumString;

type Code = Vec<KeyT>;
type Keypad<const ROWS: usize, const COLS: usize> = [[KeyT; COLS]; ROWS];
type PointT = i16;
type Point = (PointT, PointT);
type OutT = u64;

#[derive(EnumString, Eq, PartialEq, Hash, Clone, Copy)]
enum KeyT {
    Empty,
    #[strum(serialize = "A")]
    Press,
    Number(u8),
    #[strum(serialize = "<")]
    Left,
    #[strum(serialize = ">")]
    Right,
    #[strum(serialize = "^")]
    Up,
    #[strum(serialize = "v")]
    Down,
}

fn parse(input: &str) -> Vec<Code> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c.is_digit(10) {
                        KeyT::Number(c.to_digit(10).unwrap() as u8)
                    } else {
                        KeyT::from_str(&c.to_string()).unwrap()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn code_to_number(code: &Code) -> OutT {
    code.iter()
        .filter_map(|key| match key {
            KeyT::Number(n) => Some(*n),
            _ => None,
        })
        .fold(0, |acc, n| acc * 10 + n as OutT)
}

fn prepend_press<'a>(code: &'a [KeyT]) -> impl Iterator<Item = &'a KeyT> {
    std::iter::once(&KeyT::Press).chain(code.iter())
}

fn is_in_bounds<const ROWS: usize, const COLS: usize>(
    _: Keypad<ROWS, COLS>,
    (x, y): Point,
) -> bool {
    x >= 0 && y >= 0 && x < COLS as PointT && y < ROWS as PointT
}

const NUM_KEYPAD: Keypad<4, 3> = [
    [KeyT::Number(7), KeyT::Number(8), KeyT::Number(9)],
    [KeyT::Number(4), KeyT::Number(5), KeyT::Number(6)],
    [KeyT::Number(1), KeyT::Number(2), KeyT::Number(3)],
    [KeyT::Empty, KeyT::Number(0), KeyT::Press],
];

const DIR_KEYPAD: Keypad<2, 3> = [
    [KeyT::Empty, KeyT::Up, KeyT::Press],
    [KeyT::Left, KeyT::Down, KeyT::Right],
];

const KEYPAD_DIRS: [(i16, i16, KeyT); 4] = [
    (0, -1, KeyT::Up),
    (0, 1, KeyT::Down),
    (-1, 0, KeyT::Left),
    (1, 0, KeyT::Right),
];

fn keypad_positions<const ROWS: usize, const COLS: usize>(
    keypad: Keypad<ROWS, COLS>,
) -> FxHashMap<KeyT, Point> {
    keypad
        .iter()
        .enumerate()
        .fold(FxHashMap::default(), |mut acc, (row, keys)| {
            for (col, key) in keys.iter().enumerate() {
                acc.insert(*key, (col as PointT, row as PointT));
            }
            acc
        })
}

fn keypad_shortest_paths_len(
    shortest_paths: &FxHashMap<(Point, Point), Vec<Code>>,
) -> FxHashMap<(Point, Point), usize> {
    shortest_paths
        .iter()
        .map(|((from, to), paths)| {
            (
                (*from, *to),
                paths.iter().min_by_key(|path| path.len()).unwrap().len(),
            )
        })
        .collect()
}

fn keypad_shortest_paths<const ROWS: usize, const COLS: usize>(
    keypad: Keypad<ROWS, COLS>,
    positions: &FxHashMap<KeyT, Point>,
) -> FxHashMap<(Point, Point), Vec<Code>> {
    positions
        .values()
        .filter(|(x, y)| keypad[*y as usize][*x as usize] != KeyT::Empty)
        .cartesian_product(
            positions
                .values()
                .filter(|(x, y)| keypad[*y as usize][*x as usize] != KeyT::Empty),
        )
        .fold(FxHashMap::default(), |mut acc, (from, to)| {
            if from == to {
                acc.insert((*from, *to), vec![vec![KeyT::Press]]);
                return acc; // aleviate indentation a little bit
            }

            let mut queue = VecDeque::new();
            let mut best_score = usize::MAX;
            let mut best_paths = vec![];
            queue.push_front((*from, vec![]));

            while let Some((point, path)) = queue.pop_back() {
                let (x, y) = point;
                for (dx, dy, key) in KEYPAD_DIRS.iter() {
                    let (new_x, new_y) = (x + dx, y + dy);

                    if !is_in_bounds(keypad, (new_x, new_y))
                        || keypad[new_y as usize][new_x as usize] == KeyT::Empty
                    {
                        continue;
                    }

                    let new_score = path.len() + 1;
                    if (new_x, new_y) == *to {
                        if new_score > best_score {
                            queue.clear();
                            break; // we are done
                        } else {
                            best_score = new_score;
                            best_paths.push({
                                let mut new_path = path.clone();
                                new_path.extend([*key, KeyT::Press]);
                                new_path
                            });
                        }
                    } else {
                        let new_path = {
                            let mut new_path = path.clone();
                            new_path.push(*key);
                            new_path
                        };
                        queue.push_front(((new_x, new_y), new_path));
                    }
                }
            }
            acc.insert((*from, *to), best_paths);

            acc
        })
}

fn shortest_paths_for_code(
    code: Code,
    positions: &FxHashMap<KeyT, Point>,
    shortest_paths: &FxHashMap<(Point, Point), Vec<Code>>,
) -> Vec<Code> {
    prepend_press(&code)
        .tuple_windows()
        .map(|(from, to)| {
            let from_pos = positions.get(from).unwrap();
            let to_pos = positions.get(to).unwrap();

            shortest_paths.get(&(*from_pos, *to_pos)).unwrap()
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|inner| inner.iter().cloned())
        .multi_cartesian_product()
        .map(|combination| combination.into_iter().flatten().collect::<Vec<_>>()) // Flatten each combination
        .collect::<Vec<_>>()
}

// NOTE for memoization to work properly without having to clone alll of the
// hashmaps, we assume that we will be accepting only directional keypad
#[memoize(Ignore: keypad, Ignore: positions, Ignore: shortest_paths, Ignore: shortest_paths_len, CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn code_solution_len<const ROWS: usize, const COLS: usize>(
    code: Code,
    robot: usize,
    keypad: Keypad<ROWS, COLS>,
    positions: &FxHashMap<KeyT, Point>,
    shortest_paths: &FxHashMap<(Point, Point), Vec<Code>>,
    shortest_paths_len: &FxHashMap<(Point, Point), usize>,
) -> usize {
    let tuples = prepend_press(&code).tuple_windows();
    if robot == 1 {
        return tuples
            .map(|(from, to)| {
                let from_pos = positions.get(from).unwrap();
                let to_pos = positions.get(to).unwrap();

                shortest_paths_len.get(&(*from_pos, *to_pos)).unwrap()
            })
            .sum::<usize>();
    }

    tuples
        .par_bridge()
        .map(|(from, to)| {
            let from_pos = positions.get(from).unwrap();
            let to_pos = positions.get(to).unwrap();

            shortest_paths
                .get(&(*from_pos, *to_pos))
                .unwrap()
                .par_iter()
                .map(|path| {
                    code_solution_len(
                        path.clone(),
                        robot - 1,
                        keypad,
                        positions,
                        shortest_paths,
                        shortest_paths_len,
                    )
                })
                .min()
                .unwrap()
        })
        .sum::<usize>()
}

fn solve(input: &str, robots: usize) -> Option<OutT> {
    let codes = parse(input);
    // Uhhh, just precompute everything
    let num_keypad_positions = keypad_positions(NUM_KEYPAD);
    let dir_keypad_positions = keypad_positions(DIR_KEYPAD);
    let num_keypad_paths = keypad_shortest_paths(NUM_KEYPAD, &num_keypad_positions);
    let dir_keypad_paths = keypad_shortest_paths(DIR_KEYPAD, &dir_keypad_positions);
    // let num_keypad_paths_len = keypad_shortest_paths_len(&num_keypad_paths);
    let dir_keypad_paths_len = keypad_shortest_paths_len(&dir_keypad_paths);

    Some(
        codes
            .par_iter()
            .map(|code| {
                (shortest_paths_for_code(code.clone(), &num_keypad_positions, &num_keypad_paths)
                    .par_iter()
                    .map(|sol| {
                        code_solution_len(
                            sol.to_owned(),
                            robots,
                            DIR_KEYPAD,
                            &dir_keypad_positions,
                            &dir_keypad_paths,
                            &dir_keypad_paths_len,
                        )
                    })
                    .min()
                    .unwrap() as OutT)
                    * code_to_number(code)
            })
            .sum::<OutT>(),
    )
}

pub fn part_one(input: &str) -> Option<OutT> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<OutT> {
    solve(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154115708116294));
    }
}
