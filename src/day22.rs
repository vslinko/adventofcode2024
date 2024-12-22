use std::simd::prelude::*;

macro_rules! read_unsigned_skip {
    ($input:expr, $i:expr) => {{
        let mut num = 0;
        loop {
            match $input.get_unchecked($i) {
                b'0'..=b'9' => {
                    num = num * 10 + ($input.get_unchecked($i) - b'0') as i64;
                    $i += 1;
                }
                _ => {
                    $i += 1;
                    break;
                }
            }
        }
        num
    }};
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
        (0..64).for_each(|j| {
            if i < input.len() {
                *nums.get_unchecked_mut(j) = read_unsigned_skip!(input, i);
            } else {
                *nums.get_unchecked_mut(j) = 0;
            }
        });

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

        (0..2000).for_each(|_| {
            mix!(nums * _64);
            prune!();
            mix!(nums / _32);
            prune!();
            mix!(nums * _2048);
            prune!();
        });

        result += nums.reduce_sum();
    }

    result
}

pub fn part2(input: &str) -> i64 {
    unsafe { inner2(input) }
}

#[cfg_attr(
    target_arch = "x86_64",
    target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")
)]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner2(input: &str) -> i64 {
    let input = input.as_bytes();
    let mut i = 0;

    let _0 = isizex64::splat(0);
    let _9 = i64x64::splat(9);
    let _10 = i64x64::splat(10);
    let _15 = i64x64::splat(15);
    let _19 = i64x64::splat(19);
    let _32 = i64x64::splat(32);
    let _37 = i64x64::splat(37);
    let _42 = i64x64::splat(42);
    let _64 = i64x64::splat(64);
    let _361 = i64x64::splat(361);
    let _2048 = i64x64::splat(2048);
    let _6859 = i64x64::splat(6859);
    let _16113920 = i64x64::splat(16113920);
    let _16777216 = i64x64::splat(16777216);
    let _100000000 = i64x64::splat(100000000);
    let bitpos = isizex64::from_array(std::array::from_fn(|i| 1 << i));

    let mut results_map = vec![0; 130321];
    let mut already_done = vec![0_isize; 130321];

    let mut nums = [0; 64];

    while i < input.len() {
        (0..64).for_each(|j| {
            if i < input.len() {
                *nums.get_unchecked_mut(j) = read_unsigned_skip!(input, i);
            } else {
                *nums.get_unchecked_mut(j) = 0;
            }
        });

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

        let mut prev = nums % _10;
        let mut new_price;
        let mut diff;

        macro_rules! next {
            () => {
                mix!(nums * _64);
                prune!();
                mix!(nums / _32);
                prune!();
                mix!(nums * _2048);
                prune!();
                new_price = nums % _10;
                diff = new_price - prev;
                prev = new_price;
            };
        }

        next!();
        let mut a = diff;

        next!();
        let mut b = diff;

        next!();
        let mut c = diff;

        next!();
        let mut d = diff;

        macro_rules! remember_seq {
            () => {
                let hash = (a + _9) + (b + _9) * _19 + (c + _9) * _361 + (d + _9) * _6859;
                let hash: usizex64 = hash.cast();
                let already_done_vals = isizex64::gather_or_default(&already_done, hash);
                let unfilled = (already_done_vals & bitpos).simd_eq(_0);
                (already_done_vals | bitpos).scatter(&mut already_done, hash);

                unfilled
                    .to_array()
                    .iter()
                    .zip(hash.as_array().iter())
                    .zip(new_price.as_array().iter())
                    .filter(|((unfilled, _), _)| **unfilled)
                    .for_each(|((_, hash), new_price)| {
                        *results_map.get_unchecked_mut(*hash) += new_price;
                    })
            };
        }

        remember_seq!();

        (4..2000).for_each(|_| {
            next!();
            (a, b, c, d) = (b, c, d, diff);
            remember_seq!();
        });

        already_done.fill(0);
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
