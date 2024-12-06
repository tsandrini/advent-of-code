advent_of_code::solution!(6);

use itertools::Itertools;
use std::collections::HashSet;

struct GridCombinations {
    grid: Vec<Vec<char>>,
    positions: Vec<(usize, usize)>,
    current: usize,
}

impl GridCombinations {
    fn new(grid: &str) -> Self {
        let grid: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();
        let positions = grid
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(col, &c)| if c == '.' { Some((row, col)) } else { None })
            })
            .collect();
        GridCombinations {
            grid,
            positions,
            current: 0,
        }
    }
}

impl Iterator for GridCombinations {
    type Item = Vec<Vec<char>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.positions.len() {
            return None;
        }

        let (row, col) = self.positions[self.current];
        self.current += 1;

        // Clone the grid and modify the specific cell
        let mut new_grid = self.grid.clone();
        new_grid[row][col] = '#';

        Some(new_grid)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (guard_pos, grid) = parse(input);
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);

    let mut direction = (0, -1); // Starting ^
    let mut pos = guard_pos;
    let mut visited = HashSet::new();
    visited.insert(pos);

    loop {
        let new_pos = (pos.0 + direction.0, pos.1 + direction.1);

        if new_pos.0 < 0 || new_pos.0 >= cols || new_pos.1 < 0 || new_pos.1 >= rows {
            break;
        } else if grid[new_pos.1 as usize][new_pos.0 as usize] == '#' {
            direction = (-direction.1, direction.0); // 2d rotation matrix for 90 degrees
        } else {
            pos = new_pos;
            visited.insert(pos);
        }
    }

    Some(visited.len())
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

pub fn part_two(input: &str) -> Option<u32> {
    let (guard_pos, grid) = parse(input);
    let (rows, cols) = (grid.len() as isize, grid[0].len() as isize);

    Some(
        GridCombinations::new(input)
            .filter(|grid| is_cyclic(grid, guard_pos, (cols, rows)))
            .count() as u32,
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
