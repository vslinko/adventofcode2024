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

macro_rules! iter {
    ($number:expr) => {
        $number = prune(mix($number, $number * 64));
        $number = prune(mix($number, $number / 32));
        $number = prune(mix($number, $number * 2048));
    };
}

fn iterate(number: i64, iters: usize) -> i64 {
    let mut number = number;

    for _ in 0..iters {
        iter!(number);
    }

    number
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| iterate(line.parse::<i64>().unwrap(), 2000))
        .sum()
}

fn seq_hash(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a + b * 100 + c * 10000 + d * 1000000
}

pub fn part2(input: &str) -> i64 {
    let mut results_map: FxHashMap<i64, i64> =
        FxHashMap::with_capacity_and_hasher(50000, FxBuildHasher::default());
    let mut already_done: FxHashSet<i64> =
        FxHashSet::with_capacity_and_hasher(2000, FxBuildHasher::default());

    let mut diff_seq: [i64; 4] = [0, 0, 0, 0];

    macro_rules! remember_seq {
        ($diff_seq:expr, $new_price:expr) => {
            let seq = seq_hash($diff_seq[0], $diff_seq[1], $diff_seq[2], $diff_seq[3]);

            if !already_done.contains(&seq) {
                already_done.insert(seq);

                results_map
                    .entry(seq)
                    .and_modify(|e| *e += $new_price)
                    .or_insert($new_price);
            }
        };
    }

    macro_rules! iter2 {
        ($number:expr, $prev:expr, |$new_price:ident| $block:block) => {
            iter!($number);
            let $new_price = $number % 10;
            $block
            $prev = $new_price;
        };
    }

    for line in input.lines() {
        let mut number = line.parse::<i64>().unwrap();
        let mut prev = number % 10;

        iter2!(number, prev, |new_price| {
            diff_seq[0] = new_price - prev;
        });
        iter2!(number, prev, |new_price| {
            diff_seq[1] = new_price - prev;
        });
        iter2!(number, prev, |new_price| {
            diff_seq[2] = new_price - prev;
        });
        iter2!(number, prev, |new_price| {
            diff_seq[3] = new_price - prev;
            remember_seq!(diff_seq, new_price);
        });

        for _ in 4..2000 {
            iter2!(number, prev, |new_price| {
                diff_seq[0] = diff_seq[1];
                diff_seq[1] = diff_seq[2];
                diff_seq[2] = diff_seq[3];
                diff_seq[3] = new_price - prev;
                remember_seq!(diff_seq, new_price);
            });
        }

        already_done.clear();
    }

    *results_map.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day22_part1() {
        let prod_input = read_to_string("./inputs/22.txt").unwrap();
        let prod_output = read_to_string("./outputs/22p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day22_part2() {
        let prod_input = read_to_string("./inputs/22.txt").unwrap();
        let prod_output = read_to_string("./outputs/22p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
