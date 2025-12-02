advent_of_code::solution!(2);

use rustc_hash::FxHashSet;

type ResT = u64;

fn parse(input: &str) -> impl Iterator<Item = (ResT, ResT)> {
    input
        .trim()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .map(|(lo, hi)| (lo.parse::<ResT>().unwrap(), hi.parse::<ResT>().unwrap()))
}

fn repeat_factor(base: ResT, r: u32) -> ResT {
    // 1 + base + base^2 + ... + base^(r-1)
    (0..r)
        .fold((0, 1), |(acc, term), _| (acc + term, term * base))
        .0
}

pub fn part_one(input: &str) -> Option<ResT> {
    Some(
        parse(input)
            .flat_map(|(lo, hi)| {
                let dk_max = hi.ilog10().div_ceil(2);
                let dk_min = (lo.ilog10() + 1).div_ceil(2);

                (dk_min..=dk_max).flat_map(move |k| {
                    let pow10k = 10u64.pow(k);
                    let half_lo = pow10k / 10; // 10^(k-1)
                    let half_hi = pow10k - 1; // 10^k - 1
                    let factor = pow10k + 1; // 10^k + 1

                    let mut i_min = ResT::div_ceil(lo, factor);
                    let mut i_max = hi / factor;

                    if i_min < half_lo {
                        i_min = half_lo;
                    }
                    if i_max > half_hi {
                        i_max = half_hi;
                    }

                    (i_min..=i_max).map(move |i| i * factor)
                })
            })
            .sum::<ResT>(),
    )
}

pub fn part_two(input: &str) -> Option<ResT> {
    Some(
        parse(input)
            .flat_map(|(lo, hi)| {
                let dlo = lo.ilog10() + 1;
                let dhi = hi.ilog10() + 1;

                (2..=dhi)
                    .flat_map(move |r| {
                        let k_min = dlo.div_ceil(r);
                        let k_max = dhi / r;

                        (k_min..=k_max).flat_map(move |k| {
                            let base = 10u64.pow(k); // 10^k
                            let half_lo = base / 10; // 10^(k-1)
                            let half_hi = base - 1; // 10^k - 1
                            let factor = repeat_factor(base, r);

                            let mut x_min = ResT::div_ceil(lo, factor);
                            let mut x_max = hi / factor;

                            if x_min < half_lo {
                                x_min = half_lo;
                            }
                            if x_max > half_hi {
                                x_max = half_hi;
                            }

                            (x_min..=x_max).map(move |x| x * factor)
                        })
                    })
                    .fold(FxHashSet::default(), |mut acc, y| {
                        acc.insert(y);
                        acc
                    })
            })
            .sum::<ResT>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
