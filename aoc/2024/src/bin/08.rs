advent_of_code::solution!(8);

use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

fn parse(input: &str) -> (Vec<Vec<char>>, (i32, i32), FxHashMap<char, Vec<(i32, i32)>>) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let (rows, cols) = (grid.len() as i32, grid[0].len() as i32);

    let cache: FxHashMap<char, Vec<(i32, i32)>> = grid
        .par_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.par_iter().enumerate().filter_map(move |(j, &cell)| {
                if cell != '.' {
                    Some((cell, (i as i32, j as i32)))
                } else {
                    None
                }
            })
        })
        .fold(FxHashMap::default, |mut acc, (cell, coord)| {
            acc.entry(cell).or_insert_with(Vec::new).push(coord);
            acc
        })
        .reduce(FxHashMap::default, |mut acc, map| {
            for (cell, coords) in map {
                acc.entry(cell).or_insert_with(Vec::new).extend(coords);
            }
            acc
        });

    (grid, (rows, cols), cache)
}

fn gen_vector_of_antinodes_in_direction(
    start: &(i32, i32),
    direction: (i32, i32),
    size: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut out = vec![];

    let mut antinode = *start;
    while antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < size.0 && antinode.1 < size.1 {
        out.push(antinode);
        antinode = (antinode.0 + direction.0, antinode.1 + direction.1);
    }

    out
}

pub fn part_one(input: &str) -> Option<i32> {
    let (_, (rows, cols), cache) = parse(input);

    Some(
        cache
            .iter()
            .flat_map(|(cell, coords)| {
                let l = coords.len();
                if l <= 1 {
                    vec![]
                } else {
                    coords
                        .into_iter()
                        .permutations(2)
                        .par_bridge()
                        .map(|pair| {
                            let x_dist = pair[0].0 - pair[1].0;
                            let y_dist = pair[0].1 - pair[1].1;
                            let antinode = (pair[0].0 + x_dist, pair[0].1 + y_dist);
                            if antinode.0 < 0
                                || antinode.1 < 0
                                || antinode.0 >= rows
                                || antinode.1 >= cols
                            {
                                (antinode, false)
                            } else {
                                (antinode, true)
                            }
                        })
                        .collect::<Vec<_>>()
                }
            })
            .filter(|(_, is_antinode)| *is_antinode)
            .map(|(coord, _)| coord)
            .unique()
            .count() as i32,
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, (rows, cols), cache) = parse(input);

    Some(
        cache
            .iter()
            .flat_map(|(cell, coords)| {
                let l = coords.len();
                if l <= 1 {
                    vec![]
                } else {
                    coords
                        .into_iter()
                        .combinations(2)
                        .par_bridge()
                        .flat_map(|pair| {
                            vec![
                                gen_vector_of_antinodes_in_direction(
                                    &pair[0],
                                    ((pair[1].0 - pair[0].0), (pair[1].1 - pair[0].1)),
                                    (rows, cols),
                                ),
                                gen_vector_of_antinodes_in_direction(
                                    &pair[1],
                                    ((pair[0].0 - pair[1].0), (pair[0].1 - pair[1].1)),
                                    (rows, cols),
                                ),
                                gen_vector_of_antinodes_in_direction(
                                    &pair[1],
                                    ((pair[1].0 - pair[0].0), (pair[1].1 - pair[0].1)),
                                    (rows, cols),
                                ),
                            ]
                        })
                        .collect::<Vec<_>>()
                }
            })
            .flatten()
            .unique()
            .count() as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
