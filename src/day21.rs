use std::mem::transmute;
use std::simd::prelude::*;

const LUT1: [u64; 1000] = unsafe { transmute(*include_bytes!("../luts/day21_1.bin")) };
const LUT2: [u64; 1000] = unsafe { transmute(*include_bytes!("../luts/day21_2.bin")) };

#[cfg_attr(
    target_arch = "x86_64",
    target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")
)]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn solve(input: &str, lut: &[u64]) -> u64 {
    let input = input.as_bytes();

    let u1 = usizex8::splat(1);
    let u10 = usizex8::splat(10);
    let u100 = usizex8::splat(100);
    let u5328 = usizex8::splat(5328); // 100 * 48 + 10 * 48 + 48

    let first_buttons_idxs = usizex8::from_array([0, 5, 10, 15, 20, 25, 30, 35]);
    let second_buttons_idxs = first_buttons_idxs + u1;
    let third_buttons_idxs = second_buttons_idxs + u1;

    let first_buttons = u8x8::gather_or_default(&input, first_buttons_idxs);
    let second_buttons = u8x8::gather_or_default(&input, second_buttons_idxs);
    let third_buttons = u8x8::gather_or_default(&input, third_buttons_idxs);

    let first_buttons = first_buttons.cast();
    let second_buttons = second_buttons.cast();
    let third_buttons = third_buttons.cast();

    let num = first_buttons * u100 + second_buttons * u10 + third_buttons - u5328;

    u64x8::gather_or_default(lut, num).reduce_sum()
}

pub fn part1(input: &str) -> u64 {
    unsafe { solve(input, &LUT1) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { solve(input, &LUT2) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day21_part1() {
        let prod_input = read_to_string("./inputs/21.txt").unwrap();
        let prod_output = read_to_string("./outputs/21p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day21_part2() {
        let prod_input = read_to_string("./inputs/21.txt").unwrap();
        let prod_output = read_to_string("./outputs/21p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
