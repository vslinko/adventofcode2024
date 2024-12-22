use std::simd::prelude::*;

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

unsafe fn parse(input: &str) -> i64 {
    input.parse::<i64>().unwrap_unchecked()
}

pub fn part1(input: &str) -> i64 {
    unsafe { inner1(input) }
}

#[cfg_attr(
    target_arch = "x86_64",
    target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")
)]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner1(input: &str) -> i64 {
    let mut initial_numbers = input.lines().map(|line| parse(line)).collect::<Vec<_>>();
    while initial_numbers.len() % 64 != 0 {
        initial_numbers.push(0);
    }

    let _15 = i64x64::splat(15);
    let _32 = i64x64::splat(32);
    let _37 = i64x64::splat(37);
    let _42 = i64x64::splat(42);
    let _64 = i64x64::splat(64);
    let _2048 = i64x64::splat(2048);
    let _16113920 = i64x64::splat(16113920);
    let _16777216 = i64x64::splat(16777216);
    let _100000000 = i64x64::splat(100000000);

    macro_rules! mix {
        ($a:expr, $b:expr) => {{
            let b = $b;
            let xored = ($a ^ $b).to_array();
            let use_xored = $a.simd_ne(_42) | b.simd_ne(_15);
            i64x64::load_select(&xored, use_xored, _37)
        }};
    }

    macro_rules! prune {
        ($a:expr) => {{
            let moduled = ($a % _16777216).to_array();
            let use_moduled = $a.simd_ne(_100000000);
            i64x64::load_select(&moduled, use_moduled, _16113920)
        }};
    }

    let mut result = 0;
    let mut i = 0;

    while i < initial_numbers.len() {
        let mut nums = i64x64::from_slice(&initial_numbers[i..i + 64]);

        for _ in 0..2000 {
            nums = prune!(mix!(nums, nums * _64));
            nums = prune!(mix!(nums, nums / _32));
            nums = prune!(mix!(nums, nums * _2048));
        }

        result += nums.reduce_sum();
        i += 64;
    }

    result
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
        let mut number = parse(line);
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

                if !already_done.get_unchecked(hash) {
                    *already_done.get_unchecked_mut(hash) = true;
                    *results_map.get_unchecked_mut(hash) += $new_price;
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
