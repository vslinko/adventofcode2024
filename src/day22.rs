fn mix(a: i64, b: i64) -> i64 {
    match (a, b) {
        (42, 15) => 37,
        _ => a ^ b,
    }
}

fn prune(a: i64) -> i64 {
    match a {
        100000000 => 16113920,
        _ => a % 16777216,
    }
}

fn iter(number: i64) -> i64 {
    let mut number = number;
    number = prune(mix(number, number * 64));
    number = prune(mix(number, number / 32));
    prune(mix(number, number * 2048))
}

pub fn part1(input: &str) -> i64 {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut number = line.parse::<i64>().unwrap_unchecked();

            for _ in 0..2000 {
                number = iter(number);
            }

            number
        })
        .sum()
}

fn seq_hash(a: i64, b: i64, c: i64, d: i64) -> usize {
    ((a + 9) + (b + 9) * 19 + (c + 9) * 361 + (d + 9) * 6859) as usize
}

pub fn part2(input: &str) -> i64 {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> i64 {
    let mut results_map: [i64; 130321] = [0; 130321];

    macro_rules! iter2 {
        ($number:expr, $prev:expr, $new_price:expr, $diff:expr) => {
            $number = iter($number);
            $new_price = $number % 10;
            $diff = $new_price - $prev;
            $prev = $new_price;
        };
    }

    for line in input.lines() {
        let mut already_done: [bool; 130321] = [false; 130321];
        let mut number = line.parse::<i64>().unwrap_unchecked();
        let mut prev = number % 10;

        #[allow(unused_assignments)]
        let mut new_price = 0;
        #[allow(unused_assignments)]
        let mut diff = 0;
        #[allow(unused_assignments)]
        let mut a = 0;
        #[allow(unused_assignments)]
        let mut b = 0;
        #[allow(unused_assignments)]
        let mut c = 0;
        #[allow(unused_assignments)]
        let mut d = 0;

        macro_rules! remember_seq {
            ($a:expr, $b:expr, $c:expr, $d:expr, $new_price:expr) => {
                let hash = seq_hash($a, $b, $c, $d);

                if !already_done[hash] {
                    already_done[hash] = true;
                    results_map[hash] += $new_price;
                }
            };
        }

        iter2!(number, prev, new_price, diff);
        a = diff;

        iter2!(number, prev, new_price, diff);
        b = diff;

        iter2!(number, prev, new_price, diff);
        c = diff;

        iter2!(number, prev, new_price, diff);
        d = diff;

        remember_seq!(a, b, c, d, new_price);

        for _ in 4..2000 {
            iter2!(number, prev, new_price, diff);
            a = b;
            b = c;
            c = d;
            d = diff;
            remember_seq!(a, b, c, d, new_price);
        }
    }

    *results_map.iter().max().unwrap_unchecked()
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
