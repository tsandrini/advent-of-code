advent_of_code::solution!(2);

fn state_value(state: char) -> u32 {
    match state {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => 0,
    }
}

fn eval_game(game: &[(u32, u32)]) -> Option<u32> {
    Some(
        game.iter()
            .map(|(b, a)| match (*a).abs_diff(*b) {
                1 => {
                    if *a > *b {
                        *a + 6
                    } else {
                        *a
                    }
                }
                2 => {
                    if *a < *b {
                        *a + 6
                    } else {
                        *a
                    }
                }
                0 => *a + 3,
                _ => 0,
            })
            .sum(),
    )
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            (
                state_value(line.as_bytes()[0] as char),
                state_value(line.as_bytes()[2] as char),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    eval_game(&parse(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let game: Vec<(u32, u32)> = parse(input)
        .iter()
        .map(|(a, b)| match (a, b) {
            (1, 1) => (1, 3),
            (1, 3) => (1, 2),
            (3, 1) => (3, 2),
            (3, 3) => (3, 1),
            (x, 2) => (*x, *x),
            (x, y) => (*x, *y),
        })
        .collect();
    eval_game(&game)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }
}
