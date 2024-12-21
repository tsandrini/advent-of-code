advent_of_code::solution!(20);

use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::str::FromStr;
use strum_macros::EnumString;

type PointT = i16;
type Point = (PointT, PointT);
type Grid = Vec<Vec<GridT>>;
const DIRS: [(i16, i16); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const TWODIST_DIRS: [(i16, i16); 8] = [
    (0, 2),
    (1, 1),
    (2, 0),
    (1, -1),
    (0, -2),
    (-1, -1),
    (-2, 0),
    (-1, 1),
];

#[derive(EnumString, Eq, PartialEq, PartialOrd, Ord)]
enum GridT {
    #[strum(serialize = "#")]
    Wall,
    #[strum(serialize = ".")]
    Empty,
    #[strum(serialize = "S")]
    Start,
    #[strum(serialize = "E")]
    End,
}

fn manhttan_distance((x1, y1): Point, (x2, y2): Point) -> u32 {
    ((x1 - x2).abs() + (y1 - y2).abs()) as u32
}

fn is_in_bounds((x, y): Point, (width, height): (usize, usize)) -> bool {
    x >= 0 && x < width as PointT && y >= 0 && y < height as PointT
}

fn parse(input: &str) -> (Grid, Point, Point, (usize, usize)) {
    let mut start_pos = (0 as PointT, 0 as PointT);
    let mut end_pos = (0 as PointT, 0 as PointT);

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let grid_t = GridT::from_str(&c.to_string()).unwrap();
                    match grid_t {
                        GridT::Start => start_pos = (x as PointT, y as PointT),
                        GridT::End => end_pos = (x as PointT, y as PointT),
                        _ => (),
                    }
                    grid_t
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    let (width, height) = (grid[0].len(), grid.len());
    (grid, start_pos, end_pos, (width, height))
}

fn get_reference_path(grid: &Grid, start_pos: Point, end_pos: Point) -> Vec<Point> {
    let size = (grid[0].len(), grid.len());
    let mut hist = vec![];

    // Only one path so this is fine
    let (mut x, mut y) = start_pos;
    let (mut dx, mut dy) = DIRS
        .iter()
        .find(|(dx, dy)| {
            let (new_x, new_y) = (x + dx, y + dy);
            is_in_bounds((new_x, new_y), size)
                && grid[new_y as usize][new_x as usize] != GridT::Wall
        })
        .unwrap();

    while (x, y) != end_pos {
        hist.push((x, y));
        for (new_dx, new_dy) in [(dx, dy), (dy, -dx), (-dy, dx)].into_iter() {
            let (new_x, new_y) = (x + new_dx, y + new_dy);

            if is_in_bounds((new_x, new_y), size)
                && grid[new_y as usize][new_x as usize] != GridT::Wall
            {
                (x, y, dx, dy) = (new_x, new_y, new_dx, new_dy);
                break;
            }
        }
    }
    hist.push(end_pos);

    hist
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, start_pos, end_pos, _) = parse(input);
    let path = get_reference_path(&grid, start_pos, end_pos);

    let indexed_path = path
        .iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<FxHashMap<_, _>>();

    Some(path.iter().fold(0, |out_acc, &(x, y)| {
        let curr_cost = *indexed_path.get(&(x, y)).unwrap();
        TWODIST_DIRS.iter().fold(out_acc, |acc, &(dx, dy)| {
            let new_pos = (x + dx, y + dy);
            if let Some(&new_cost) = indexed_path.get(&new_pos) {
                if new_cost > curr_cost && (new_cost - curr_cost) >= 102 {
                    return acc + 1;
                }
            }
            acc
        })
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, start_pos, end_pos, _) = parse(input);
    let path = get_reference_path(&grid, start_pos, end_pos);

    let indexed_path = path
        .iter()
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect::<FxHashMap<_, _>>();

    let dists = path
        .iter()
        .enumerate()
        .fold(FxHashMap::default(), |acc, (i, &pos)| {
            path.iter().skip(i + 1).fold(acc, |mut acc, &new_pos| {
                let dist = manhttan_distance(pos, new_pos) as usize;
                acc.insert((pos, new_pos), dist);
                acc
            })
        });

    Some(
        path.par_iter()
            .enumerate()
            .map(|(i, &pos)| {
                let (x, y) = pos;
                let curr_cost = *indexed_path.get(&(x, y)).unwrap();

                path.par_iter()
                    .skip(i + 1)
                    .map(|&new_pos| {
                        let (new_x, new_y) = new_pos;
                        let new_cost = *indexed_path.get(&(new_x, new_y)).unwrap();
                        let dist = *dists.get(&(pos, new_pos)).unwrap();
                        if dist <= 20 && (new_cost - curr_cost) >= (100 + dist) {
                            1
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
