advent_of_code::solution!(6);

use itertools::Itertools;
use std::collections::HashSet;

fn compute_traversal(
    grid: &Vec<Vec<char>>,
    guard_pos: (isize, isize),
    size: (isize, isize),
) -> HashSet<(isize, isize)> {
    let mut direction = (0, -1); // Starting ^
    let mut pos = guard_pos;
    let mut visited = HashSet::new();
    visited.insert(pos);

    loop {
        let new_pos = (pos.0 + direction.0, pos.1 + direction.1);

        if new_pos.0 < 0 || new_pos.0 >= size.0 || new_pos.1 < 0 || new_pos.1 >= size.1 {
            break;
        } else if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            direction = (-direction.1, direction.0); // 2d rotation matrix for 90 degrees
        } else {
            pos = new_pos;
            visited.insert(pos);
        }
    }

    visited
}

fn is_cyclic(grid: &Vec<Vec<char>>, guard_pos: (isize, isize), size: (isize, isize)) -> bool {
    let mut direction = (0, -1); // Starting ^
    let mut pos = guard_pos;
    let mut visited = HashSet::new();
    visited.insert((pos, direction));

    loop {
        let new_pos = (pos.0 + direction.0, pos.1 + direction.1);

        if new_pos.0 < 0 || new_pos.0 >= size.0 || new_pos.1 < 0 || new_pos.1 >= size.1 {
            break;
        } else if visited.contains(&(new_pos, direction)) {
            return true;
        } else if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            direction = (-direction.1, direction.0); // 2d rotation matrix for 90 degrees
        } else {
            pos = new_pos;
            visited.insert((pos, direction));
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    let (guard_pos, grid) = parse(input);
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);

    Some(compute_traversal(&grid, guard_pos, (cols, rows)).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (guard_pos, mut grid) = parse(input);
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);

    Some(
        compute_traversal(&grid, guard_pos, (cols, rows))
            .iter()
            .filter(|&pos| {
                grid[pos.1 as usize][pos.0 as usize] = '#';
                let x = is_cyclic(&grid, guard_pos, (cols, rows));
                grid[pos.1 as usize][pos.0 as usize] = '.';
                x
            })
            .count(),
    )
}

fn parse(input: &str) -> ((isize, isize), Vec<Vec<char>>) {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let idx = input
        .chars()
        .filter(|&c| c != '\n')
        .collect::<String>()
        .find('^')
        .unwrap();
    let cols = input.lines().next().unwrap().len();
    let pos = ((idx % cols) as isize, (idx / cols) as isize);

    grid[pos.0 as usize][pos.1 as usize] = '.';

    (pos, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
