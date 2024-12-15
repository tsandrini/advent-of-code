advent_of_code::solution!(15);

type GridT = u8; // 0 = #, 1 = ., 2 = O, 3 = @, 4 = [, 5 = ]
type MovT = u8; // 0 = <, 1 = ^, 2 = >, 3 = v
type PosT = u8;
type Pos = (PosT, PosT);
const DIRS: [(i8, i8); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)]; // in order of MovT
type Grid = Vec<Vec<GridT>>;

fn parse(input: &str, wide_grid: bool) -> (Grid, Vec<MovT>, Pos) {
    let (grid_s, movs_s) = input.split_once("\n\n").unwrap();
    let mut robot_pos = (0, 0);
    let movs = movs_s
        .lines()
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '<' => 0,
                    '^' => 1,
                    '>' => 2,
                    'v' => 3,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let grid = if !wide_grid {
        grid_s
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        '#' => 0,
                        '.' => 1,
                        'O' => 2,
                        '@' => {
                            robot_pos = (j as PosT, i as PosT);
                            1
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    } else {
        grid_s
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .flat_map(|(j, c)| match c {
                        '#' => [0, 0],
                        '.' => [1, 1],
                        'O' => [4, 5],
                        '@' => {
                            robot_pos = (2 * j as PosT, i as PosT);
                            [1, 1]
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };

    (grid, movs, robot_pos)
}

pub fn part_one(input: &str) -> Option<u16> {
    let (mut grid, movs, mut robot_pos) = parse(input, false);
    for mov in movs {
        let (dx, dy) = DIRS[mov as usize];
        let (x, y) = robot_pos;
        let (new_x, new_y) = (x as i8 + dx, y as i8 + dy);

        match grid[new_y as usize][new_x as usize] {
            1 => {
                robot_pos = (new_x as PosT, new_y as PosT);
            }
            2 => {
                let (mut final_x, mut final_y) = (new_x, new_y);
                while grid[final_y as usize][final_x as usize] == 2 {
                    final_x += dx;
                    final_y += dy;
                }
                if grid[final_y as usize][final_x as usize] == 1 {
                    grid[final_y as usize][final_x as usize] = 2;
                    grid[new_y as usize][new_x as usize] = 1;
                    robot_pos = (new_x as PosT, new_y as PosT);
                }
            }
            _ => {}
        }
    }

    Some(grid.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (j, &cell)| {
            acc + if cell == 2 { (100 * i + j) as u16 } else { 0 }
        })
    }))
}

fn can_move(grid: &Grid, pos: Pos, dir: (i8, i8)) -> bool {
    let (x, y) = pos;
    let (dx, dy) = dir;
    let (new_x, new_y) = (x as i8 + dx, y as i8 + dy);

    match grid[new_y as usize][new_x as usize] {
        0 => false,
        1 => true,
        4 | 5 => match (dx, dy) {
            (-1, 0) | (1, 0) => {
                let mut final_x = new_x;
                while grid[new_y as usize][final_x as usize] == 4
                    || grid[new_y as usize][final_x as usize] == 5
                {
                    final_x += dx;
                }
                grid[new_y as usize][final_x as usize] == 1
            }
            (0, -1) | (0, 1) => {
                let box_type = grid[new_y as usize][new_x as usize];
                let left_block_x = if box_type == 4 { new_x } else { new_x - 1 };

                can_move(grid, (left_block_x as PosT, new_y as PosT), (0, dy))
                    && can_move(grid, ((left_block_x + 1) as PosT, new_y as PosT), (0, dy))
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

// NOTE the function assumes that the move is valid
fn move_blocks(grid: &mut Grid, pos: Pos, dir: (i8, i8)) {
    let (x, y) = pos;
    let (dx, dy) = dir;
    let (new_x, new_y) = (x as i8 + dx, y as i8 + dy);

    if grid[y as usize][x as usize] <= 1 {
        return;
    }

    match (dx, dy) {
        (-1, 0) | (1, 0) => {
            let mut final_x = new_x;

            while grid[y as usize][final_x as usize] == 4 || grid[y as usize][final_x as usize] == 5
            {
                final_x += dx;
            }

            let mut tmp_x = final_x;
            while tmp_x != x as i8 {
                grid[y as usize][tmp_x as usize] = grid[y as usize][(tmp_x - dx) as usize];
                tmp_x -= dx; // GOING BACKWARDS
            }
            grid[y as usize][x as usize] = 1;
        }
        (0, -1) | (0, 1) => {
            let box_type = grid[y as usize][x as usize];
            let left_block_x = if box_type == 4 { x } else { x - 1 };

            move_blocks(grid, (left_block_x as PosT, (new_y) as PosT), (0, dy));
            move_blocks(grid, ((left_block_x + 1) as PosT, (new_y) as PosT), (0, dy));

            grid[new_y as usize][left_block_x as usize] = 4;
            grid[new_y as usize][(left_block_x + 1) as usize] = 5;
            grid[y as usize][left_block_x as usize] = 1;
            grid[y as usize][(left_block_x + 1) as usize] = 1;
        }
        _ => {}
    }
}

pub fn part_two(input: &str) -> Option<u16> {
    let (mut grid, movs, mut robot_pos) = parse(input, true);
    for mov in movs {
        let (dx, dy) = DIRS[mov as usize];
        let (x, y) = robot_pos;
        let (new_x, new_y) = (x as i8 + dx, y as i8 + dy);

        if can_move(&grid, robot_pos, (dx, dy)) {
            move_blocks(&mut grid, (new_x as PosT, new_y as PosT), (dx, dy));
            robot_pos = (new_x as PosT, new_y as PosT);
        }
    }

    Some(grid.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row.iter().enumerate().fold(0, |acc, (j, &cell)| {
            acc + if cell == 4 { (100 * i + j) as u16 } else { 0 }
        })
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
