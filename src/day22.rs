use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet};

fn mix(a: i64, b: i64) -> i64 {
    if a == 42 && b == 15 {
        return 37;
    }

    a ^ b
}

fn prune(a: i64) -> i64 {
    if a == 100000000 {
        return 16113920;
    }

    a % 16777216
}

fn iterate(number: i64, iters: usize) -> i64 {
    let mut number = number;

    for _ in 0..iters {
        number = prune(mix(number, number * 64));
        number = prune(mix(number, number / 32));
        number = prune(mix(number, number * 2048));
    }

    number
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| iterate(line.parse::<i64>().unwrap(), 2000))
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let mut results_map: FxHashMap<(i64, i64, i64, i64), i64> =
        FxHashMap::with_capacity_and_hasher(130321, FxBuildHasher::default());
    let mut already_done: FxHashSet<(i64, i64, i64, i64)> =
        FxHashSet::with_capacity_and_hasher(130321, FxBuildHasher::default());

    let mut diff_seq = Vec::with_capacity(4);

    for line in input.lines() {
        let mut number = line.parse::<i64>().unwrap();
        let mut prev = number % 10;

        for _ in 0..2000 {
            number = prune(mix(number, number * 64));
            number = prune(mix(number, number / 32));
            number = prune(mix(number, number * 2048));

            let new_price = number % 10;

            if diff_seq.len() == 4 {
                diff_seq[0] = diff_seq[1];
                diff_seq[1] = diff_seq[2];
                diff_seq[2] = diff_seq[3];
                diff_seq[3] = new_price - prev;
            } else {
                diff_seq.push(new_price - prev);
            }

            if diff_seq.len() == 4 {
                let seq = (diff_seq[0], diff_seq[1], diff_seq[2], diff_seq[3]);

                if !already_done.contains(&seq) {
                    already_done.insert(seq);

                    results_map
                        .entry(seq)
                        .and_modify(|e| *e += new_price)
                        .or_insert(new_price);
                }
            }

            prev = new_price;
        }

        diff_seq.clear();
        already_done.clear();
    }

    *results_map.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const TEST_INPUT1: &str = "1
10
100
2024";

    const TEST_INPUT2: &str = "1
2
3
2024";

    #[test]
    fn test_day22_part1() {
        assert_eq!(part1(&TEST_INPUT1).to_string(), "37327623");
        let prod_input = read_to_string("./inputs/22.txt").unwrap();
        let prod_output = read_to_string("./outputs/22p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day22_part2() {
        assert_eq!(part2(&TEST_INPUT2).to_string(), "23");
        let prod_input = read_to_string("./inputs/22.txt").unwrap();
        let prod_output = read_to_string("./outputs/22p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
