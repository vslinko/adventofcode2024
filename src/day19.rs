use rayon::prelude::*;
use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let mut lines = input.lines();

    let patterns_by_length: FxHashMap<usize, Vec<&str>> =
        lines.next().unwrap_unchecked().split(',').fold(
            FxHashMap::with_capacity_and_hasher(8, FxBuildHasher::default()),
            |mut acc, pattern| {
                let pattern = pattern.trim_start();
                acc.entry(pattern.len()).or_default().push(pattern);
                acc
            },
        );

    lines.next();

    let designs: Vec<&str> = lines.collect();

    designs
        .par_iter()
        .filter(|design| can_make_design(design, &patterns_by_length))
        .count()
}

fn can_make_design_recursive(
    pos: usize,
    design: &[u8],
    patterns_by_length: &FxHashMap<usize, Vec<&str>>,
    memo: &mut FxHashSet<usize>,
) -> bool {
    if pos == design.len() {
        return true;
    }

    if memo.contains(&pos) {
        return false;
    }

    for (&length, patterns) in patterns_by_length.iter() {
        let next_pos = pos + length;

        if next_pos > design.len() {
            continue;
        }

        'pattern_loop: for pattern in patterns {
            for (j, &pattern_byte) in pattern.as_bytes().iter().enumerate() {
                if design[pos + j] != pattern_byte {
                    continue 'pattern_loop;
                }
            }

            if can_make_design_recursive(next_pos, design, patterns_by_length, memo) {
                return true;
            }
        }
    }

    memo.insert(pos);
    false
}

fn can_make_design(design: &str, patterns_by_length: &FxHashMap<usize, Vec<&str>>) -> bool {
    let design_bytes = design.as_bytes();
    let mut memo = FxHashSet::with_hasher(FxBuildHasher::default());
    can_make_design_recursive(0, design_bytes, patterns_by_length, &mut memo)
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> u64 {
    let mut lines = input.lines();

    let patterns_by_length: FxHashMap<usize, Vec<&str>> =
        lines.next().unwrap_unchecked().split(',').fold(
            FxHashMap::with_capacity_and_hasher(8, FxBuildHasher::default()),
            |mut acc, pattern| {
                let pattern = pattern.trim_start();
                acc.entry(pattern.len()).or_default().push(pattern);
                acc
            },
        );

    lines.next();

    let designs: Vec<&str> = lines.collect();

    designs
        .iter()
        .map(|design| count_possible_combinations(design, &patterns_by_length))
        .sum()
}

fn count_possible_combinations(
    design: &str,
    patterns_by_length: &FxHashMap<usize, Vec<&str>>,
) -> u64 {
    let design_bytes = design.as_bytes();
    let mut dp = vec![0; design_bytes.len() + 1];
    dp[design_bytes.len()] = 1;

    for i in (0..design_bytes.len()).rev() {
        for (&length, patterns) in patterns_by_length.iter() {
            if design_bytes.len() - i < length {
                continue;
            }

            'pattern_loop: for pattern in patterns {
                let pattern_bytes = pattern.as_bytes();

                for (j, &pattern_byte) in pattern_bytes.iter().enumerate() {
                    if design_bytes[i + j] != pattern_byte {
                        continue 'pattern_loop;
                    }
                }

                if dp[i + length] > 0 {
                    dp[i] += dp[i + length];
                    break;
                }
            }
        }
    }

    dp[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day19_part1() {
        let prod_input = read_to_string("./inputs/19.txt").unwrap();
        let prod_output = read_to_string("./outputs/19p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day19_part2() {
        let prod_input = read_to_string("./inputs/19.txt").unwrap();
        let prod_output = read_to_string("./outputs/19p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
