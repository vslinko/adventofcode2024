use std::simd::cmp::SimdPartialEq;
use std::simd::f64x32;
use std::simd::num::SimdFloat;
use std::simd::StdFloat;

macro_rules! read2 {
    ($input:expr, $i:expr) => {{
        let mut num = *$input.get_unchecked($i) as f64 * 10.0;
        num += *$input.get_unchecked($i + 1) as f64;
        num - 528.0 // 528 = b'0' * 10 + b'0'
    }};
}

macro_rules! read_case {
    ($input:expr, $i:expr, $ax:expr, $ay:expr, $bx:expr, $by:expr, $px:expr, $py:expr) => {{
        $i += 12; // skip "Button A: X+"
        $ax = read2!($input, $i);
        $i += 6; // skip number and ", Y+"
        $ay = read2!($input, $i);
        $i += 15; // skip number and "\nButton B: X+"
        $bx = read2!($input, $i);
        $i += 6; // skip number and ", Y+"
        $by = read2!($input, $i);
        $i += 12; // skip number and "\nPrize: X="
        $px = read!($input, $i);
        $i += 4; // skip ", Y="
        $py = read!($input, $i);
        $i += 2; // skip "\n\n"
    }};
}

macro_rules! read {
    ($input:expr, $i:expr) => {{
        let mut num = 0.0;
        loop {
            match $input.get_unchecked($i) {
                b'0'..=b'9' => {
                    num = num * 10.0 + ($input.get_unchecked($i) - b'0') as f64;
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

macro_rules! solution {
    ( $input:expr $(, $delta:expr )? ) => {
        unsafe {
            let input = $input.as_bytes();
            let mut i = 0;
            let mut sum = 0.0;
            let mut ax = f64x32::splat(0.0);
            let mut ay = f64x32::splat(0.0);
            let mut bx = f64x32::splat(0.0);
            let mut by = f64x32::splat(0.0);
            let mut px = f64x32::splat(0.0);
            let mut py = f64x32::splat(0.0);

            let three = f64x32::splat(3.0);
            let zero = f64x32::splat(0.0);

            while i < input.len() {
                read_case!(input, i, ax[0], ay[0], bx[0], by[0], px[0], py[0]);
                read_case!(input, i, ax[1], ay[1], bx[1], by[1], px[1], py[1]);
                read_case!(input, i, ax[2], ay[2], bx[2], by[2], px[2], py[2]);
                read_case!(input, i, ax[3], ay[3], bx[3], by[3], px[3], py[3]);
                read_case!(input, i, ax[4], ay[4], bx[4], by[4], px[4], py[4]);
                read_case!(input, i, ax[5], ay[5], bx[5], by[5], px[5], py[5]);
                read_case!(input, i, ax[6], ay[6], bx[6], by[6], px[6], py[6]);
                read_case!(input, i, ax[7], ay[7], bx[7], by[7], px[7], py[7]);
                read_case!(input, i, ax[8], ay[8], bx[8], by[8], px[8], py[8]);
                read_case!(input, i, ax[9], ay[9], bx[9], by[9], px[9], py[9]);
                read_case!(input, i, ax[10], ay[10], bx[10], by[10], px[10], py[10]);
                read_case!(input, i, ax[11], ay[11], bx[11], by[11], px[11], py[11]);
                read_case!(input, i, ax[12], ay[12], bx[12], by[12], px[12], py[12]);
                read_case!(input, i, ax[13], ay[13], bx[13], by[13], px[13], py[13]);
                read_case!(input, i, ax[14], ay[14], bx[14], by[14], px[14], py[14]);
                read_case!(input, i, ax[15], ay[15], bx[15], by[15], px[15], py[15]);
                read_case!(input, i, ax[16], ay[16], bx[16], by[16], px[16], py[16]);
                read_case!(input, i, ax[17], ay[17], bx[17], by[17], px[17], py[17]);
                read_case!(input, i, ax[18], ay[18], bx[18], by[18], px[18], py[18]);
                read_case!(input, i, ax[19], ay[19], bx[19], by[19], px[19], py[19]);
                read_case!(input, i, ax[20], ay[20], bx[20], by[20], px[20], py[20]);
                read_case!(input, i, ax[21], ay[21], bx[21], by[21], px[21], py[21]);
                read_case!(input, i, ax[22], ay[22], bx[22], by[22], px[22], py[22]);
                read_case!(input, i, ax[23], ay[23], bx[23], by[23], px[23], py[23]);
                read_case!(input, i, ax[24], ay[24], bx[24], by[24], px[24], py[24]);
                read_case!(input, i, ax[25], ay[25], bx[25], by[25], px[25], py[25]);
                read_case!(input, i, ax[26], ay[26], bx[26], by[26], px[26], py[26]);
                read_case!(input, i, ax[27], ay[27], bx[27], by[27], px[27], py[27]);
                read_case!(input, i, ax[28], ay[28], bx[28], by[28], px[28], py[28]);
                read_case!(input, i, ax[29], ay[29], bx[29], by[29], px[29], py[29]);
                read_case!(input, i, ax[30], ay[30], bx[30], by[30], px[30], py[30]);
                read_case!(input, i, ax[31], ay[31], bx[31], by[31], px[31], py[31]);

                $(
                    let delta = f64x32::splat($delta);
                    let px = px + delta;
                    let py = py + delta;
                )?

                let cross = ax * by - ay * bx;
                let a_count = ((px * by - py * bx) / cross).round();
                let b_count = ((py * ax - px * ay) / cross).round();

                let good = (ax * a_count + bx * b_count).simd_eq(px) & (ay * a_count + by * b_count).simd_eq(py);
                let tokens = a_count * three + b_count;

                sum += good.select(tokens, zero).reduce_sum();
            }

            sum as u64
        }
    };
}

pub fn part1(input: &str) -> u64 {
    solution!(input)
}

pub fn part2(input: &str) -> u64 {
    solution!(input, 10000000000000.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day13_part1() {
        let prod_input = read_to_string("./inputs/13.txt").unwrap();
        let prod_output = read_to_string("./outputs/13p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day13_part2() {
        let prod_input = read_to_string("./inputs/13.txt").unwrap();
        let prod_output = read_to_string("./outputs/13p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
