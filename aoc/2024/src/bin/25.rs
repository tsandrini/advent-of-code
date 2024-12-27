advent_of_code::solution!(25);

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn part_one(input: &str) -> Option<u16> {
    let (locks_s, keys_s): (Vec<_>, Vec<_>) = input.split("\n\n").partition(|s| s.starts_with("#"));

    let locks = locks_s
        .iter()
        .map(|lock| {
            let grid = lock
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let grid_t = transpose(&grid);

            grid_t
                .into_iter()
                .map(|row| row.into_iter().take_while(|c| *c == '#').count() - 1)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let keys = keys_s
        .iter()
        .map(|key| {
            let grid = key
                .lines()
                .rev()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            let grid_t = transpose(&grid);

            grid_t
                .into_iter()
                .map(|row| row.into_iter().take_while(|c| *c == '#').count() - 1)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Some(locks.iter().fold(0, |acc, lock| {
        keys.iter().fold(acc, |acc, key| {
            acc + lock
                .iter()
                .zip(key.iter())
                .map(|(l, k)| k + l)
                .all(|n| n <= 5) as u16
        })
    }))
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
