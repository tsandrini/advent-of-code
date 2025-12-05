advent_of_code::solution!(3);

use rayon::prelude::*;

type ResT = u64;

#[allow(dead_code)]
pub fn part_one_attempt_one(input: &str) -> Option<ResT> {
    Some(
        input
            .lines()
            .map(|line| {
                line.bytes()
                    .rev()
                    .map(|b| b.wrapping_sub(b'0'))
                    .fold((None::<(u8, u8)>, None::<u8>), |(best, suf_max), d| {
                        let best = match suf_max {
                            None => best,
                            Some(m) => {
                                let cand = (d, m);
                                match best {
                                    None => Some(cand),
                                    Some(b) => {
                                        let cv = 10u64 * cand.0 as ResT + cand.1 as ResT;
                                        let bv = 10u64 * b.0 as ResT + b.1 as ResT;
                                        if cv > bv { Some(cand) } else { Some(b) }
                                    }
                                }
                            }
                        };
                        (best, Some(suf_max.map_or(d, |m| m.max(d))))
                    })
                    .0
                    .unwrap()
            })
            .map(|(a, b)| 10u64 * a as ResT + b as ResT)
            .sum::<ResT>(),
    )
}

fn solve(input: &str, n: usize) -> ResT {
    input
        .par_lines()
        .map(|line| {
            line.bytes()
                .fold((Vec::new(), line.len() - n), |(mut st, mut del), b| {
                    while del > 0 && st.last().is_some_and(|&top| top < b) {
                        st.pop();
                        del -= 1;
                    }
                    st.push(b);
                    (st, del)
                })
                .0
                .into_iter()
                .take(n)
                .fold(0, |acc, b| 10 * acc + b.wrapping_sub(b'0') as ResT)
        })
        .sum::<ResT>()
}

pub fn part_one(input: &str) -> Option<ResT> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<ResT> {
    Some(solve(input, 12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
