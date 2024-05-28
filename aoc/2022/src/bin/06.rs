advent_of_code::solution!(6);

fn idx_of_n_consecutive_chars(input: &str, n: usize) -> Option<u32> {
    let mut counter: [u8; 26] = [0; 26];
    let mut buff: Vec<char> = Vec::new();

    for (i, c) in input.chars().enumerate() {
        if !c.is_ascii_lowercase() {
            continue; // Skip non-lowercase ASCII characters
        }

        let idx = (c as u8 - b'a') as usize;
        if counter[idx] > 0 {
            while let Some(&first) = buff.first() {
                buff.remove(0);
                counter[(first as u8 - b'a') as usize] -= 1;
                if first == c {
                    break;
                }
            }
        }

        buff.push(c);
        counter[idx] += 1;

        if buff.len() == n {
            return Some((i + 1) as u32);
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    idx_of_n_consecutive_chars(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    idx_of_n_consecutive_chars(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));

        assert_eq!(part_one(&"bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one(&"nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));

        assert_eq!(part_two(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(part_two(&"bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_two(&"nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_two(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_two(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
