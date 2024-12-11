advent_of_code::solution!(9);

type FsBlockType = u64;
const FS_FREE_SPACE_PT1: FsBlockType = FsBlockType::MAX;
const FS_FREE_SPACE_PT2: FsBlockType = 0;

pub fn part_one(input: &str) -> Option<FsBlockType> {
    let mut fs = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            vec![
                if i % 2 == 0 {
                    (i / 2) as FsBlockType
                } else {
                    FS_FREE_SPACE_PT1
                };
                c.to_digit(10).unwrap() as usize
            ]
        })
        .collect::<Vec<_>>();

    let mut i = fs.len() - 1;
    let mut leftmost_free = fs.iter().position(|&c| c == FS_FREE_SPACE_PT1).unwrap();
    while leftmost_free < i {
        fs.swap(i, leftmost_free);

        while leftmost_free < i && fs[leftmost_free] != FS_FREE_SPACE_PT1 {
            leftmost_free += 1;
        }
        i -= 1;
    }

    fs.truncate(i + 1);
    Some(
        fs.iter()
            .enumerate()
            .fold(0, |acc, (i, &c)| acc + i as FsBlockType * c),
    )
}

pub fn part_two(input: &str) -> Option<FsBlockType> {
    // NOTE use 0 as FS_FREE_SPACE_PT2 to avoid having to merge consecutive blocks
    // of free space
    let mut fs = input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            (
                if i % 2 == 0 {
                    (i as FsBlockType / 2) + 1
                } else {
                    FS_FREE_SPACE_PT2
                },
                c.to_digit(10).unwrap() as usize,
            )
        })
        .collect::<Vec<_>>();

    let mut i = fs.len() - 1;
    while i > 0 {
        if fs[i].0 == FS_FREE_SPACE_PT2 {
            i -= 1;
            continue;
        }

        for leftmost_free in 0..i {
            // we need to do the check here due to shifting indices
            let (target_val, target_size) = fs[i];
            let (free_val, free_size) = fs[leftmost_free];

            if target_val != FS_FREE_SPACE_PT2
                && free_val == FS_FREE_SPACE_PT2
                && target_size <= free_size
            {
                fs[i] = (FS_FREE_SPACE_PT2, target_size);
                fs[leftmost_free] = (FS_FREE_SPACE_PT2, free_size - target_size);
                fs.insert(leftmost_free, (target_val, target_size));
            }
        }
        i -= 1;
    }

    Some(
        fs.iter()
            .flat_map(|(val, size)| vec![val; *size])
            .enumerate()
            .fold(0, |acc, (i, c)| {
                acc + if *c == FS_FREE_SPACE_PT2 {
                    0
                } else {
                    i as FsBlockType * (*c - 1)
                }
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
