advent_of_code::solution!(12);

use petgraph::graph::Graph;
use petgraph::visit::Dfs;
use petgraph::Undirected;
use rustc_hash::FxHashMap;

type GraphT = i16;
type CornerT = f32;
const DIRS: [(GraphT, GraphT); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
const CORNER_DIRS: [(CornerT, CornerT); 4] = [(-0.5, -0.5), (0.5, -0.5), (-0.5, 0.5), (0.5, 0.5)];

fn is_in_bounds(i: GraphT, j: GraphT, rows: usize, cols: usize) -> bool {
    i >= 0 && i < rows as GraphT && j >= 0 && j < cols as GraphT
}

fn unique_tuples_with_tolerance(
    vec: Vec<(CornerT, CornerT)>,
    tolerance: CornerT,
) -> Vec<(CornerT, CornerT)> {
    let mut unique_vec: Vec<(CornerT, CornerT)> = Vec::new();
    for &item in &vec {
        if !unique_vec.iter().any(|&unique_item| {
            (item.0 - unique_item.0).abs() < tolerance && (item.1 - unique_item.1).abs() < tolerance
        }) {
            unique_vec.push(item);
        }
    }
    unique_vec
}

fn parse(
    input: &str,
) -> (
    Vec<Vec<char>>,
    (usize, usize),
    Graph<char, (), Undirected>,
    FxHashMap<usize, (GraphT, GraphT)>,
) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (rows, cols) = (grid.len(), grid[0].len());
    let mut graph = Graph::new_undirected();
    let mut node_map = FxHashMap::default();

    let node_grid = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, curr_group)| {
                    let node = graph.add_node(*curr_group);
                    node_map.insert(node.index(), (i as GraphT, j as GraphT));
                    Some(node)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, curr_group)| {
            let curr_node = node_grid[i][j].unwrap();

            DIRS.iter()
                .map(|(di, dj)| (i as GraphT + di, j as GraphT + dj))
                .filter(|&(x, y)| is_in_bounds(x, y, rows, cols))
                .for_each(|(x, y)| {
                    if grid[x as usize][y as usize] == *curr_group {
                        let neighbor_node = node_grid[x as usize][y as usize].unwrap();
                        graph.add_edge(curr_node, neighbor_node, ());
                    }
                });
        });
    });

    (grid, (rows, cols), graph, node_map)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, (rows, cols), graph, node_map) = parse(input);

    let mut visited = vec![false; graph.node_count()];
    let mut price = 0;

    for start in graph.node_indices() {
        if visited[start.index()] {
            continue;
        }

        let mut area = 0;
        let mut perimeter = 0;
        let mut dfs = Dfs::new(&graph, start);

        while let Some(node) = dfs.next(&graph) {
            let (x, y) = node_map[&node.index()];
            visited[node.index()] = true;

            area += 1;
            perimeter += DIRS
                .iter()
                .filter(|&(dx, dy)| {
                    let (test_x, test_y) = (x + dx, y + dy);
                    !is_in_bounds(test_x, test_y, rows, cols)
                        || grid[test_x as usize][test_y as usize] != graph[node]
                })
                .count();
        }
        price += area * perimeter;
    }

    Some(price)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, _, graph, node_map) = parse(input);

    let mut visited = vec![false; graph.node_count()];
    let mut price = 0;

    for start in graph.node_indices() {
        if visited[start.index()] {
            continue;
        }

        let mut area = 0;
        let mut group = vec![];
        let mut dfs = Dfs::new(&graph, start);
        let mut group_corners = vec![];

        while let Some(node) = dfs.next(&graph) {
            let (x, y) = node_map[&node.index()];

            visited[node.index()] = true;
            area += 1;
            group.push(node_map[&node.index()]);

            group_corners.extend(
                CORNER_DIRS
                    .iter()
                    .map(|&(dx, dy)| ((x as CornerT + dx), (y as CornerT + dy))),
            );
        }

        let sides = unique_tuples_with_tolerance(group_corners, 0.3)
            .iter()
            .map(|(x, y)| {
                let adj_corners = CORNER_DIRS
                    .iter()
                    .map(|&(dx, dy)| {
                        let (test_x, test_y) = (
                            (*x as CornerT + dx) as GraphT,
                            (*y as CornerT + dy) as GraphT,
                        );

                        group.contains(&(test_x, test_y))
                    })
                    .collect::<Vec<_>>();

                match adj_corners.iter().filter(|&&x| x).count() {
                    3 | 1 => 1,
                    2 => {
                        if matches!(
                            adj_corners.as_slice(),
                            [true, false, false, true] | [false, true, true, false]
                        ) {
                            2 // opposite corners -> side
                        } else {
                            0 // same side -> adjacent corners -> not a side
                        }
                    }
                    _ => 0,
                }
            })
            .sum::<usize>();

        price += area * sides;
    }

    Some(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
