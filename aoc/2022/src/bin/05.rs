advent_of_code::solution!(5);

use itertools::Itertools;

fn skip_last<T>(mut iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let last = iter.next();
    iter.scan(last, |state, item| std::mem::replace(state, Some(item)))
}

fn pop_multiple<T>(stack: &mut Vec<T>, amount: u32) -> Vec<T> {
    let mut result = Vec::new();
    for _ in 0..amount {
        result.push(stack.pop().unwrap());
    }
    result
}

fn push_multiple<T>(stack: &mut Vec<T>, items: Vec<T>) {
    for item in items.into_iter() {
        stack.push(item);
    }
}

struct CratesInfo {
    stacks: Vec<Vec<char>>,
    actions: Vec<(u32, u32, u32)>,
}

fn parse_input(input: &str) -> CratesInfo {
    let (crates_str, actions_str) = input.split("\n\n").tuples().next().unwrap();
    let actions: Vec<_> = actions_str
        .lines()
        .map(|line| {
            line.split(' ')
                .filter(|s| s.chars().all(|c| c.is_numeric()))
                .map(|x| x.parse::<u32>().unwrap())
                .tuples::<(_, _, _)>()
                .next()
                .unwrap()
        })
        .collect();

    let crates = skip_last(crates_str.lines())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let stacks_num = (crates_str.lines().next_back().unwrap().len() + 1) / 4;
    let layers_num = crates.len();
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stacks_num);

    for _ in 0..stacks_num {
        stacks.push(Vec::new());
    }

    for (i, stack) in stacks.iter_mut().enumerate().take(stacks_num) {
        for layer in (0..layers_num).rev() {
            let c = crates[layer][4 * i + 1];
            if c == ' ' {
                break;
            }
            stack.push(c);
        }
    }

    CratesInfo { stacks, actions }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut info = parse_input(input);

    for (amount, src, target) in info.actions {
        let src = src as usize - 1;
        let target = target as usize - 1;
        let buffer = pop_multiple(&mut info.stacks[src], amount);

        push_multiple(info.stacks[target].as_mut(), buffer);
    }

    let mut result = String::new();

    for mut stack in info.stacks {
        result.push(stack.pop().unwrap());
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut info = parse_input(input);

    for (amount, src, target) in info.actions {
        let src = src as usize - 1;
        let target = target as usize - 1;
        let mut buffer = pop_multiple(&mut info.stacks[src], amount);

        buffer.reverse();

        push_multiple(info.stacks[target].as_mut(), buffer);
    }

    let mut result = String::new();

    for mut stack in info.stacks {
        result.push(stack.pop().unwrap());
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("MCD")));
    }
}
