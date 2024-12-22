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

macro_rules! read_unsigned {
    ($input:expr, $i:expr) => {{
        let mut num = 0;
        loop {
            match $input.get_unchecked($i) {
                b'0'..=b'9' => {
                    num = num * 10 + ($input.get_unchecked($i) - b'0') as i64;
                    $i += 1;
                }
                _ => {
                    break;
                }
            }
        }
        num
    }};
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
    let input = input.as_bytes();
    let mut i = 0;

    let _15 = i64x64::splat(15);
    let _32 = i64x64::splat(32);
    let _37 = i64x64::splat(37);
    let _42 = i64x64::splat(42);
    let _64 = i64x64::splat(64);
    let _2048 = i64x64::splat(2048);
    let _16113920 = i64x64::splat(16113920);
    let _16777216 = i64x64::splat(16777216);
    let _100000000 = i64x64::splat(100000000);

    let mut result = 0;
    let mut nums = [0; 64];

    while i < input.len() {
        for j in 0..64 {
            if i < input.len() {
                *nums.get_unchecked_mut(j) = read_unsigned!(input, i);
                i += 1;
            } else {
                *nums.get_unchecked_mut(j) = 0;
            }
        }

        let mut nums = i64x64::from_slice(&nums);

        macro_rules! mix {
            ($expr:expr) => {{
                let tmp = $expr;
                let xored = (nums ^ tmp).to_array();
                let use_xored = nums.simd_ne(_42) | tmp.simd_ne(_15);
                nums = i64x64::load_select(&xored, use_xored, _37)
            }};
        }

        macro_rules! prune {
            () => {{
                let moduled = (nums % _16777216).to_array();
                let use_moduled = nums.simd_ne(_100000000);
                nums = i64x64::load_select(&moduled, use_moduled, _16113920)
            }};
        }

        for _ in 0..2000 {
            mix!(nums * _64);
            prune!();
            mix!(nums / _32);
            prune!();
            mix!(nums * _2048);
            prune!();
        }

        result += nums.reduce_sum();
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
            () => {
                let hash = seq_hash(a, b, c, d);

                if !already_done.get_unchecked(hash) {
                    *already_done.get_unchecked_mut(hash) = true;
                    *results_map.get_unchecked_mut(hash) += new_price;
                }
            };
        }

        macro_rules! next {
            () => {
                number = iter(number);
                new_price = number % 10;
                diff = new_price - prev;
                prev = new_price;
            };
        }

        next!();
        a = diff;

        next!();
        b = diff;

        next!();
        c = diff;

        next!();
        d = diff;

        remember_seq!();

        for _ in 4..2000 {
            next!();
            (a, b, c, d) = (b, c, d, diff);
            remember_seq!();
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
