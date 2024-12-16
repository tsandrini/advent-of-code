// advent_of_code::solution!(16);
//
// use petgraph::algo::astar;
// use petgraph::dot::Dot;
// use petgraph::graph::{DiGraph, NodeIndex};
// use rustc_hash::FxHashSet;
// use std::collections::vec_deque::VecDeque;
//
// type PointT = i32;
// type Point = (PointT, PointT);
// const FORWARD_COST: u32 = 1; // price of moving forward
// const ROTATION_COST: u32 = 1000; // price of rotating
//
// #[allow(dead_code)]
// fn manhattan_distance(p1: Point, p2: Point) -> u32 {
//     ((p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()) as u32
// }
//
// #[allow(dead_code)]
// fn rotate_90_deg((x, y): Point, n_times: u32) -> Point {
//     (0..n_times).fold((x, y), |(x, y), _| (-y, x))
// }
//
// #[allow(dead_code)]
// fn print_path_in_grid(
//     grid: &Vec<Vec<char>>,
//     graph: &DiGraph<(Point, Point), u32>,
//     path: &Vec<NodeIndex>,
// ) {
//     let mut grid_cp = grid.clone();
//     for &p in path {
//         let ((x, y), (dx, dy)) = graph[p];
//         match (dx, dy) {
//             (1, 0) => grid_cp[y as usize][x as usize] = '>',
//             (0, -1) => grid_cp[y as usize][x as usize] = '^',
//             (-1, 0) => grid_cp[y as usize][x as usize] = '<',
//             (0, 1) => grid_cp[y as usize][x as usize] = 'v',
//             _ => panic!("Invalid direction"),
//         }
//     }
//
//     for row in grid_cp {
//         for cell in row {
//             print!("{}", cell);
//         }
//         println!();
//     }
// }
//
// #[allow(dead_code)]
// fn parse(
//     input: &str,
// ) -> (
//     Vec<Vec<char>>,
//     DiGraph<(Point, Point), u32>,
//     Point,
//     Point,
//     NodeIndex,
// ) {
//     let mut grid = input
//         .lines()
//         .map(|l| l.chars().collect::<Vec<_>>())
//         .collect::<Vec<_>>();
//     let (rows, cols) = (grid.len(), grid[0].len());
//
//     let start_pos = (1 as PointT, (rows - 2) as PointT);
//     let end_pos = ((cols - 2) as PointT, 1 as PointT);
//
//     let mut graph = DiGraph::default();
//     let mut queue = VecDeque::new();
//     let mut visited = FxHashSet::default();
//
//     let start_node = graph.add_node((start_pos, (1, 0)));
//     queue.push_back(start_node);
//
//     while let Some(node) = queue.pop_front() {
//         let ((x, y), (dx, dy)) = graph[node];
//
//         let (forw_x, forw_y) = (x + dx, y + dy);
//         if grid[forw_y as usize][forw_x as usize] != '#'
//             && !visited.contains(&((forw_x, forw_y), (dx, dy)))
//         {
//             let forward_node = graph.add_node(((forw_x, forw_y), (dx, dy)));
//             visited.insert(((forw_x, forw_y), (dx, dy)));
//             graph.add_edge(node, forward_node, FORWARD_COST);
//             queue.push_front(forward_node);
//         }
//
//         for &i in [1, 3].iter() {
//             let (rot_dx, rot_dy) = rotate_90_deg((dx, dy), i);
//
//             if !visited.contains(&((x, y), (rot_dx, rot_dy))) {
//                 let rotated_node = graph.add_node(((x, y), (rot_dx, rot_dy)));
//                 visited.insert(((x, y), (rot_dx, rot_dy)));
//
//                 graph.add_edge(node, rotated_node, ROTATION_COST);
//                 queue.push_back(rotated_node);
//             }
//         }
//     }
//
//     (grid, graph, start_pos, end_pos, start_node)
// }
//
// #[allow(dead_code)]
// pub fn part_one(_input: &str) -> Option<u32> {
//     // let (grid, graph, start_pos, end_pos, start_node) = parse(input);
//     // // This is stupid, but I am paranoid and doublechecking
//     // let (best_cost, best_path) = [(1, 0), (0, -1), (-1, 0), (0, 1)]
//     //     .iter()
//     //     .map(|dir| {
//     //         astar(
//     //             &graph,
//     //             start_node,
//     //             |finish| {
//     //                 let ((x, y), (dx, dy)) = graph[finish];
//     //                 (x, y) == end_pos && (dx, dy) == (1, 0)
//     //             },
//     //             |e| *e.weight(),
//     //             |n| {
//     //                 let ((x, y), _) = graph[n];
//     //                 manhattan_distance((x, y), end_pos)
//     //             },
//     //         )
//     //         .unwrap_or((u32::MAX, vec![]))
//     //     })
//     //     .min_by_key(|(cost, _)| *cost)
//     //     .unwrap();
//
//     // print_path_in_grid(&grid, &graph, &best_path);
//
//     // Some(best_cost)
//     None
// }
//
// #[allow(dead_code)]
// pub fn part_two(input: &str) -> Option<u32> {
//     None
// }
//
