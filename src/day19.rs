use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

#[cfg(not(day19_series))]
use rayon::prelude::*;

#[cfg(not(day19_series))]
macro_rules! iter {
    ($input:expr) => {
        $input.par_iter()
    };
}

#[cfg(day19_series)]
macro_rules! iter {
    ($input:expr) => {
        $input.iter()
    };
}

unsafe fn parse(input: &str) -> (FxHashMap<usize, Vec<&str>>, Vec<&str>) {
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

    (patterns_by_length, designs)
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let (patterns_by_length, designs) = parse(input);

    iter!(designs)
        .filter(|design| can_make_design(design, &patterns_by_length))
        .count()
}

unsafe fn can_make_design(design: &str, patterns_by_length: &FxHashMap<usize, Vec<&str>>) -> bool {
    unsafe fn can_make_design_recursive(
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
                for (j, &b) in pattern.as_bytes().iter().enumerate() {
                    if *design.get_unchecked(pos + j) != b {
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

    let mut memo = FxHashSet::with_hasher(FxBuildHasher::default());

    can_make_design_recursive(0, design.as_bytes(), patterns_by_length, &mut memo)
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> u64 {
    let (patterns_by_length, designs) = parse(input);

    iter!(designs)
        .map(|design| count_possible_combinations(design, &patterns_by_length))
        .sum()
}

unsafe fn count_possible_combinations(
    design: &str,
    patterns_by_length: &FxHashMap<usize, Vec<&str>>,
) -> u64 {
    let design = design.as_bytes();
    let mut dp = vec![0; design.len() + 1];
    *dp.get_unchecked_mut(design.len()) = 1;

    for i in (0..design.len()).rev() {
        for (&length, patterns) in patterns_by_length.iter() {
            let next_pos = i + length;

            if design.len() < next_pos {
                continue;
            }

            'pattern_loop: for pattern in patterns {
                for (j, &b) in pattern.as_bytes().iter().enumerate() {
                    if *design.get_unchecked(i + j) != b {
                        continue 'pattern_loop;
                    }
                }

                if *dp.get_unchecked(next_pos) > 0 {
                    *dp.get_unchecked_mut(i) += *dp.get_unchecked(next_pos);
                    break;
                }
            }
        }
    }

    *dp.get_unchecked(0)
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
