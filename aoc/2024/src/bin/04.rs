advent_of_code::solution!(4);

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn grid_diagonals(grid: &Vec<Vec<char>>) -> Vec<String> {
    let rows = grid.len();
    let cols = grid[0].len();

    // Top-left to bottom-right diagonals
    let mut diagonals = vec![];

    // Collect diagonals starting from top row
    for col in 0..cols {
        let mut diagonal = String::new();
        let mut i = 0;
        let mut j = col;
        while i < rows && j < cols {
            diagonal.push(grid[i][j]);
            i += 1;
            j += 1;
        }
        diagonals.push(diagonal);
    }

    // Collect diagonals starting from left column (excluding first cell since it's already covered)
    for row in 1..rows {
        let mut diagonal = String::new();
        let mut i = row;
        let mut j = 0;
        while i < rows && j < cols {
            diagonal.push(grid[i][j]);
            i += 1;
            j += 1;
        }
        diagonals.push(diagonal);
    }

    // Bottom-left to top-right diagonals
    // Collect diagonals starting from bottom row
    for col in 0..cols {
        let mut diagonal = String::new();
        let mut i = rows - 1;
        let mut j = col;
        while i < rows && j < cols {
            diagonal.push(grid[i][j]);
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
            j += 1;
        }
        diagonals.push(diagonal);
    }

    // Collect diagonals starting from left column (excluding last cell since it's already covered)
    for row in (0..rows - 1).rev() {
        let mut diagonal = String::new();
        let mut i = row;
        let mut j = 0;
        while i < rows && j < cols {
            diagonal.push(grid[i][j]);
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
            j += 1;
        }
        diagonals.push(diagonal);
    }

    diagonals
}

fn xmas_centers(grid: &Vec<Vec<char>>) -> Vec<String> {
    let mut centers = vec![];

    for row in 1..(grid.len() - 1) {
        for col in 1..(grid[0].len() - 1) {
            if grid[row][col] == 'A' {
                centers.push(String::from_iter(
                    [
                        grid[row - 1][col - 1],
                        grid[row - 1][col + 1],
                        grid[row + 1][col - 1],
                        grid[row + 1][col + 1],
                    ]
                    .iter(),
                ));
            }
        }
    }
    centers
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let input_t = transpose(&grid)
        .iter()
        .map(|l| l.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    Some(
        input.match_indices("XMAS").count()
            + input.match_indices("SAMX").count()
            + input_t.match_indices("XMAS").count()
            + input_t.match_indices("SAMX").count()
            + grid_diagonals(&grid).iter().fold(0, |acc, diag| {
                acc + diag.match_indices("XMAS").count() + diag.match_indices("SAMX").count()
            }),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Some(
        xmas_centers(&grid)
            .into_iter()
            .filter(|center| matches!(center.as_str(), "MMSS" | "SMSM" | "SSMM" | "MSMS"))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
