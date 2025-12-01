advent_of_code::solution!(18);

use itertools::Itertools;
use priority_queue::PriorityQueue;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;

type PosT = u8;
type Pos = (PosT, PosT);
type Bytes = Vec<Pos>;
type TimeT = u16;
const DIRECTIONS: [(i8, i8); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn is_in_bounds((x, y): Pos, width: usize, height: usize) -> bool {
    x < width as PosT && y < height as PosT // NOTE PosT type limits
}

fn populate_history(bytes: &Bytes) -> FxHashMap<Pos, TimeT> {
    bytes
        .iter()
        .enumerate()
        .fold(FxHashMap::default(), |mut history, (i, &(x, y))| {
            history.insert((x, y), i as TimeT);
            history
        })
}

fn parse(input: &str, example: bool) -> (Bytes, Pos, Pos, (usize, usize)) {
    let (width, height) = if example { (7, 7) } else { (71, 71) };
    let start_pos = (0, 0);
    let end_pos = ((height - 1) as PosT, (width - 1) as PosT);
    let bytes = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();

    (bytes, start_pos, end_pos, (width, height))
}

fn solve_grid(
    history: &FxHashMap<Pos, TimeT>,
    start_pos: Pos,
    end_pos: Pos,
    (width, height): (usize, usize),
    time_limit: TimeT,
) -> Option<TimeT> {
    let mut pq = PriorityQueue::new();
    let mut seen = FxHashSet::default();

    pq.push((start_pos, 0 as TimeT), Reverse(0 as TimeT));
    seen.insert(start_pos);

    while let Some(((pos, time), _)) = pq.pop() {
        let (x, y) = pos;
        seen.insert(pos);

        if pos == end_pos {
            return Some(time);
        }

        for &(dx, dy) in DIRECTIONS.iter() {
            let new_pos = ((x as i8 + dx) as PosT, (y as i8 + dy) as PosT);
            if is_in_bounds(new_pos, width, height)
                && !seen.contains(&(new_pos))
                && (!history.contains_key(&(x, y)) || (history[&(x, y)] + 1) > time_limit)
            {
                pq.push((new_pos, time + 1), Reverse(time + 1));
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<TimeT> {
    // NOTE on real input, change example=false and set time_limit=1024
    let (bytes, start_pos, end_pos, size) = parse(input, true);
    let history = populate_history(&bytes);

    solve_grid(&history, start_pos, end_pos, size, 12)
    // solve_grid(&history, start_pos, end_pos, size, 1024) // real input
}

pub fn part_two(input: &str) -> Option<String> {
    // NOTE on real input, change example=false and set time_limit=1024
    let (bytes, start_pos, end_pos, size) = parse(input, true);
    let history = populate_history(&bytes);
    let solution_time = solve_grid(&history, start_pos, end_pos, size, 11).unwrap() as usize;

    (solution_time..bytes.len())
        .into_par_iter()
        .find_map_first(|time_limit| {
            if solve_grid(&history, start_pos, end_pos, size, time_limit as TimeT).is_none() {
                let (a, b) = bytes[(time_limit - 1) as usize];
                return Some(a.to_string() + "," + &b.to_string());
            }
            None
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
