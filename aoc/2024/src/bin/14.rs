advent_of_code::solution!(14);

use itertools::Itertools;

type PointT = i64;
type Point = (PointT, PointT);
type VelocityT = i64;
type Velocity = (VelocityT, VelocityT);
type Robot = (Point, Velocity);

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Robot> + 'a {
    input.lines().map(|line| {
        let (pos_s, vel_s) = line
            .split(" ")
            .map(|part| part.split("=").last().unwrap())
            .collect_tuple()
            .unwrap();
        (
            pos_s
                .split(",")
                .map(|x| x.parse::<PointT>().unwrap())
                .collect_tuple()
                .unwrap(),
            vel_s
                .split(",")
                .map(|x| x.parse::<VelocityT>().unwrap())
                .collect_tuple()
                .unwrap(),
        )
    })
}

fn safety_factor(points: &Vec<Robot>, rows: PointT, cols: PointT) -> u32 {
    let vert_mid = cols / 2;
    let hor_mid = rows / 2;
    let mut quadrants = [0u32; 4];
    for p in points {
        let ((x, y), _) = *p;
        if x < vert_mid && y < hor_mid {
            quadrants[0] += 1;
        } else if x < vert_mid && y > hor_mid {
            quadrants[1] += 1;
        } else if x > vert_mid && y < hor_mid {
            quadrants[2] += 1;
        } else if x > vert_mid && y > hor_mid {
            quadrants[3] += 1;
        }
    }
    quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
}

fn move_robot(rb: Robot, seconds: PointT, cols: PointT, rows: PointT) -> Robot {
    let ((x, y), (dx, dy)) = rb;
    (
        (
            (x + dx * seconds).rem_euclid(cols),
            (y + dy * seconds).rem_euclid(rows),
        ),
        (dx, dy),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rows, cols, seconds) = (7, 11, 100); // EXAMPLE
                                              // let (rows, cols, seconds) = (103, 101, 100); // REAL INPUT

    Some(safety_factor(
        &parse(input)
            .map(|rb| move_robot(rb, seconds, cols, rows))
            .collect::<Vec<_>>(),
        rows,
        cols,
    ))
}

#[allow(dead_code)]
fn print_grid(points: &Vec<Point>, rows: PointT, cols: PointT) {
    let mut grid = vec![vec![0; cols as usize]; rows as usize];
    for (x, y) in points {
        grid[*y as usize][*x as usize] += 1;
    }

    for row in grid {
        for cell in row {
            let c = cell.to_string();
            print!("{}", if cell > 0 { &c } else { "." });
        }
        println!();
    }
}

#[allow(dead_code)]
fn robots_to_byte_string(rbs: &Vec<Robot>, rows: PointT, cols: PointT) -> Vec<u8> {
    let mut grid = vec![vec![0; cols as usize]; rows as usize];
    for (x, y) in rbs.iter().map(|(p, _)| *p) {
        grid[y as usize][x as usize] += 1;
    }

    grid.iter()
        .map(|row| {
            row.iter()
                .map(|cell| if *cell > 0 { *cell } else { 0 })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

pub fn part_two(_input: &str) -> Option<u32> {
    // use entropy::shannon_entropy;
    // use rustc_hash::FxHashSet;
    // Weird ahh problem, solved mostly by random propmting and looking at
    // random metrics, then plotting it. meh
    // let (rows, cols, seconds) = (7, 11, 100); // EXAMPLE
    //                                           // let (rows, cols, seconds) = (103, 101, 10000); // REAL INPUT
    // let mut cache = FxHashSet::default();
    // // let hist = vec![];
    //
    // let mut rbs = parse(input).collect::<Vec<_>>();
    // for i in 0..seconds {
    //     rbs = rbs
    //         .iter()
    //         .map(|rb| move_robot(*rb, 1, cols, rows))
    //         .collect();
    //
    //     let b = robots_to_byte_string(&rbs, rows, cols);
    //     let ent = shannon_entropy(&b);
    //     let sf = safety_factor(&rbs, rows, cols);
    //     if i % 10 == 0 {
    //         println!("i: {}, ent: {}, sf: {}", i, ent, sf)
    //     }
    //
    //     if !cache.insert(sf) {
    //         println!("Found repeating pattern at {:?} with {:?}", seconds, sf);
    //         print_grid(&rbs.iter().map(|(p, _)| *p).collect::<Vec<_>>(), rows, cols);
    //     }
    // }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
