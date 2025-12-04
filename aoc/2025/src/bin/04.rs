advent_of_code::solution!(4);

use rayon::prelude::*;

type ResT = u16;
type ElemT = u8;

pub struct Grid {
    rows: usize,
    cols: usize,
    pw: usize,
    padding: Vec<ElemT>, // (rows+2) x (cols+2)
}

impl Grid {
    pub fn from_input(input: &str) -> Self {
        let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();
        let (rows, cols) = (lines.len(), lines[0].len());

        let pw = cols + 2;
        let mut padding = vec![0; (rows + 2) * pw];

        lines.iter().enumerate().for_each(|(y, line)| {
            let base = (y + 1) * pw + 1;
            line.iter().enumerate().for_each(|(x, &c)| {
                padding[base + x] = (c == b'@') as ElemT;
            });
        });

        Self {
            rows,
            cols,
            pw,
            padding,
        }
    }

    #[inline(always)]
    fn conv2d(&self, i: usize) -> ElemT {
        self.padding[i - self.pw - 1]
            + self.padding[i - self.pw]
            + self.padding[i - self.pw + 1]
            + self.padding[i - 1]
            + self.padding[i + 1]
            + self.padding[i + self.pw - 1]
            + self.padding[i + self.pw]
            + self.padding[i + self.pw + 1]
    }

    #[inline(always)]
    pub fn step_remove_lt4(&mut self) -> usize {
        let to_remove = (0..self.rows)
            .into_par_iter()
            .map(|y| {
                let base = (y + 1) * self.pw + 1;
                (0..self.cols).fold(Vec::new(), |mut acc, x| {
                    let i = base + x;
                    if self.padding[i] == 1 && self.conv2d(i) < 4 {
                        acc.push(i);
                    }
                    acc
                })
            })
            .reduce(Vec::new, |mut a, mut b| {
                a.append(&mut b);
                a
            });

        for &i in &to_remove {
            self.padding[i] = 0;
        }

        to_remove.len()
    }
}

pub fn part_one(input: &str) -> Option<ResT> {
    let mut grid = Grid::from_input(input);
    Some(grid.step_remove_lt4() as ResT)
}

pub fn part_two(input: &str) -> Option<ResT> {
    let mut grid = Grid::from_input(input);
    let mut removed_total = 0;
    loop {
        let removed = grid.step_remove_lt4();
        removed_total += removed;
        if removed == 0 {
            break;
        }
    }
    Some(removed_total as ResT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
