advent_of_code::solution!(13);

use itertools::Itertools;
use rayon::prelude::*;

type PriceT = i64;
type PointT = i64;
type Point = (PointT, PointT);
type Machine = (Point, Point, Point);

#[allow(dead_code)]
pub fn _part_one_using_integer_programming(input: &str) -> Option<PriceT> {
    use good_lp::solvers::microlp::microlp;
    use good_lp::{constraint, variables, Solution, SolverModel};

    Some(
        parse(input)
            .par_iter()
            .map(|machine| {
                let ((a_x, a_y), (b_x, b_y), (p_x, p_y)) = *machine;

                variables! {
                    problem:
                        0 <= num_a (integer) <= 100;
                        0 <= num_b (integer) <= 100;
                }
                let solution = problem
                    .minimise(3 * num_a + num_b)
                    .using(microlp)
                    .with(constraint!(
                        num_a * (a_x as i32) + num_b * (b_x as i32) == p_x as i32
                    ))
                    .with(constraint!(
                        num_a * (a_y as i32) + num_b * (b_y as i32) == p_y as i32
                    ))
                    .solve();

                match solution {
                    Ok(solution) => {
                        let n_a = solution.value(num_a).round() as PriceT;
                        let n_b = solution.value(num_b).round() as PriceT;
                        3 * n_a + n_b
                    }
                    Err(_) => 0,
                }
            })
            .sum::<PriceT>(),
    )
}

fn solve_machine_det_matrix(machine: &Machine) -> PriceT {
    // cramer rule for Ax = b
    let ((a_x, a_y), (b_x, b_y), (p_x, p_y)) = *machine;
    let det = a_x * b_y - a_y * b_x;
    let a = (p_x * b_y - p_y * b_x) / det;
    let b = (a_x * p_y - a_y * p_x) / det;

    if a_x * a + b_x * b == p_x && a_y * a + b_y * b == p_y {
        (a * 3 + b) as PriceT
    } else {
        0
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .trim()
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .map(|line| {
                    let (_, spec) = line.split(": ").collect_tuple().unwrap();
                    spec.split(", ")
                        .map(|part| part.split(|x| x == '+' || x == '=').collect::<Vec<_>>()[1])
                        .map(|x| x.parse::<PointT>().unwrap())
                        .collect_tuple::<Point>()
                        .unwrap()
                })
                .tuples()
                .next()
                .unwrap()
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<PriceT> {
    Some(
        parse(input)
            .par_iter()
            .map(|machine| solve_machine_det_matrix(machine))
            .sum::<PriceT>(),
    )
}

pub fn part_two(input: &str) -> Option<PriceT> {
    Some(
        parse(input)
            .par_iter()
            .map(|&(btn_a, btn_b, (p_x, p_y))| {
                (btn_a, btn_b, (p_x + 10000000000000, p_y + 10000000000000))
            })
            .map(|machine| solve_machine_det_matrix(&machine))
            .sum::<PriceT>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
