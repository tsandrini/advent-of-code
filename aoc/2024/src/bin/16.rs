advent_of_code::solution!(16);

use itertools::Itertools;
use priority_queue::PriorityQueue;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::VecDeque;

type PointT = i16;
type Point = (PointT, PointT);
type EdgeT = u32;
type Grid = Vec<Vec<u8>>;
const FORWARD_COST: EdgeT = 1;
const TURN_COST: EdgeT = 1000;
const EAST: Point = (1, 0);

// For debugging
#[allow(dead_code)]
fn print_positions_in_grid(grid: &Grid, positions: &Vec<Point>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if positions.contains(&(x as PointT, y as PointT)) {
                print!("O");
            } else {
                print!("{}", if grid[y][x] == 0 { "#" } else { "." });
            }
        }
        println!();
    }
}

fn parse(input: &str) -> (Grid, Point, Point) {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { 0 } else { 1 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (width, height) = (grid[0].len(), grid.len());
    let start_point = (1, (height - 2) as PointT);
    let end_point = ((width - 2) as PointT, 1);

    (grid, start_point, end_point)
}

pub fn part_one(input: &str) -> Option<EdgeT> {
    let (grid, start_point, end_point) = parse(input);
    let mut pq = PriorityQueue::new();
    let mut seen = FxHashSet::default();

    pq.push((start_point, EAST, 0), Reverse(0));
    seen.insert((start_point, EAST));

    while let Some(((point, direction, cost), _)) = pq.pop() {
        let (x, y) = point;
        let (dx, dy) = direction;
        seen.insert((point, direction));

        if point == end_point {
            return Some(cost);
        }

        [
            ((x + dx, y + dy), (dx, dy), cost + FORWARD_COST),
            ((x, y), (-dy, dx), cost + TURN_COST), // right
            ((x, y), (dy, -dx), cost + TURN_COST), // left
        ]
        .into_iter()
        .filter(|&((p_x, p_y), dp, _)| {
            grid[p_y as usize][p_x as usize] > 0 && !seen.contains(&((p_x, p_y), dp))
        })
        .for_each(|((p_x, p_y), dp, p_cost)| {
            pq.push(((p_x, p_y), dp, p_cost), Reverse(p_cost));
        });
    }

    None
}

pub fn part_two(input: &str) -> Option<EdgeT> {
    let (grid, start_point, end_point) = parse(input);

    let mut pq = PriorityQueue::new();
    let mut came_from = FxHashMap::default();
    let mut g_score = FxHashMap::default();
    let mut best_score = EdgeT::MAX;
    let mut end_nodes = FxHashSet::default();

    g_score.insert((start_point, EAST), 0);
    pq.push((start_point, EAST, 0), Reverse(0));

    while let Some(((point, direction, cost), _)) = pq.pop() {
        let (x, y) = point;
        let (dx, dy) = direction;
        if cost > *g_score.get(&((x, y), direction)).unwrap_or(&EdgeT::MAX) {
            continue;
        }
        g_score.insert((point, direction), cost);

        if point == end_point {
            if cost > best_score {
                break;
            }
            best_score = cost;
            end_nodes.insert((point, direction));
        }
        let new_points = [
            ((x + dx, y + dy), (dx, dy), cost + FORWARD_COST),
            ((x, y), (-dy, dx), cost + TURN_COST), // right
            ((x, y), (dy, -dx), cost + TURN_COST), // left
        ];

        for &((p_x, p_y), dp, p_cost) in new_points.iter() {
            let test_score = g_score
                .get(&((p_x, p_y), dp))
                .unwrap_or(&EdgeT::MAX)
                .clone();

            if grid[p_y as usize][p_x as usize] > 0 && p_cost <= test_score {
                if p_cost < test_score {
                    came_from.insert(((p_x, p_y), dp), FxHashSet::default());
                    g_score.insert(((p_x, p_y), dp), p_cost);
                }

                came_from
                    .get_mut(&((p_x, p_y), dp))
                    .unwrap()
                    .insert((point, direction));
                pq.push(((p_x, p_y), dp, p_cost), Reverse(p_cost));
            }
        }
    }

    // process the history of optimal paths using flood fill
    let mut q = VecDeque::from_iter(end_nodes.iter().copied());
    let mut seen = FxHashSet::default();
    while let Some((point, direction)) = q.pop_front() {
        for &(prev, dprev) in came_from
            .get(&(point, direction))
            .unwrap_or(&FxHashSet::default())
            .iter()
        {
            if seen.insert((prev, dprev)) {
                q.push_back((prev, dprev));
            }
        }
    }

    Some(seen.iter().map(|&((x, y), _)| (x, y)).unique().count() as EdgeT + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
