advent_of_code::solution!(10);

use petgraph::graph::{DiGraph, NodeIndex};
use rayon::prelude::*;

type GraphT = i16;
const DIRECTIONS: [(GraphT, GraphT); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn is_in_bounds(i: GraphT, j: GraphT, rows: usize, cols: usize) -> bool {
    i >= 0 && i < rows as GraphT && j >= 0 && j < cols as GraphT
}

fn parse(input: &str) -> DiGraph<GraphT, ()> {
    let grid = input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap_or(100) as GraphT)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (rows, cols) = (grid.len(), grid[0].len());

    let mut graph = DiGraph::new();
    let mut node_map = vec![vec![None; cols]; rows];

    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, curr_height)| {
            node_map[i][j] = Some(graph.add_node(*curr_height));
        });
    });

    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, curr_height)| {
            let curr_node = node_map[i][j].unwrap();

            DIRECTIONS
                .iter()
                .map(|(di, dj)| (i as GraphT + di, j as GraphT + dj))
                .filter(|&(x, y)| is_in_bounds(x, y, rows, cols))
                .for_each(|(x, y)| {
                    if grid[x as usize][y as usize] == curr_height + 1 {
                        let neighbor_node = node_map[x as usize][y as usize].unwrap();
                        graph.add_edge(curr_node, neighbor_node, ());
                    }
                });
        });
    });

    graph
}

fn dfs_trailhead_score(
    graph: &DiGraph<GraphT, ()>,
    curr: NodeIndex,
    target: GraphT,
    visited: &mut Vec<bool>,
) -> usize {
    visited[curr.index()] = true;
    if graph[curr] == target {
        return 1;
    }

    let mut count = 0;
    for neighbor in graph.neighbors(curr) {
        if !visited[neighbor.index()] {
            count += dfs_trailhead_score(graph, neighbor, target, visited);
        }
    }

    visited[curr.index()] = false;
    count
}

fn dfs_trailhead_ranking(
    graph: &DiGraph<GraphT, ()>,
    curr: NodeIndex,
    target: GraphT,
    visited: &mut Vec<bool>,
) -> usize {
    if graph[curr] == target {
        return 1;
    }

    visited[curr.index()] = true;
    let mut count = 0;
    for neighbor in graph.neighbors(curr) {
        if !visited[neighbor.index()] {
            count += dfs_trailhead_ranking(graph, neighbor, target, visited);
        }
    }

    visited[curr.index()] = false;
    count
}

fn solve(
    input: &str,
    solver: fn(&DiGraph<GraphT, ()>, NodeIndex, GraphT, &mut Vec<bool>) -> usize,
) -> usize {
    let graph = parse(input);
    graph
        .node_indices()
        .par_bridge()
        .filter(|&node| graph[node] == 0)
        .map(|start_node| {
            let mut visited = vec![false; graph.node_count()];
            solver(&graph, start_node, 9, &mut visited)
        })
        .sum::<usize>()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, dfs_trailhead_score))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, dfs_trailhead_ranking))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
