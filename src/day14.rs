const M: i32 = 101;
const N: i32 = 103;
const MS: usize = M as usize;
const NS: usize = N as usize;
const HALF_M: i32 = M / 2;
const HALF_N: i32 = N / 2;
const PART1_SECONDS: i32 = 100;
const PART2_ROBOTS: usize = 500;
const DISPERSION_THRESHOLD: i32 = 250_000;

macro_rules! read_unsigned {
    ($input:expr, $i:expr) => {{
        let mut num = 0;
        loop {
            match $input.get_unchecked($i) {
                b'0'..=b'9' => {
                    num = num * 10 + ($input.get_unchecked($i) - b'0') as i32;
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

macro_rules! read_signed {
    ($input:expr, $i:expr) => {{
        let mut num = 0;
        let negative = if *$input.get_unchecked($i) == b'-' {
            $i += 1;
            true
        } else {
            false
        };

        loop {
            match $input.get_unchecked($i) {
                b'0'..=b'9' => {
                    num = num * 10 + ($input.get_unchecked($i) - b'0') as i32;
                    $i += 1;
                }
                _ => {
                    break;
                }
            }
        }

        if negative {
            num * -1
        } else {
            num
        }
    }};
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let input = input.as_bytes();

    let mut i = 0;
    let mut left_top = 0;
    let mut right_top = 0;
    let mut left_bottom = 0;
    let mut right_bottom = 0;

    while i < input.len() {
        i += 2; // skip "p="
        let px = read_unsigned!(input, i);
        i += 1; // skip ","
        let py = read_unsigned!(input, i);
        i += 3; // skip " v="
        let vx = read_signed!(input, i);
        i += 1; // skip ","
        let vy = read_signed!(input, i);
        i += 1; // skip "\n"

        let x = ((px + PART1_SECONDS * vx) % M + M) % M;
        let y = ((py + PART1_SECONDS * vy) % N + N) % N;

        if x < HALF_M && y < HALF_N {
            left_top += 1;
        } else if x > HALF_M && y < HALF_N {
            right_top += 1;
        } else if x < HALF_M && y > HALF_N {
            left_bottom += 1;
        } else if x > HALF_M && y > HALF_N {
            right_bottom += 1;
        }
    }

    left_top * right_top * left_bottom * right_bottom
}

pub fn part2(input: &str) -> i32 {
    unsafe { inner2(input) }
}

fn is_dispersion_big(arr: &[i32], len: i32) -> bool {
    let mut sum = 0;
    let mut sum_sq = 0;

    for &count in arr {
        sum += count;
        sum_sq += count * count;
    }

    sum_sq * len - sum * sum > DISPERSION_THRESHOLD
}

unsafe fn inner2(input: &str) -> i32 {
    let input = input.as_bytes();

    let mut robots = [(0, 0, 0, 0); PART2_ROBOTS];
    let mut i = 0;
    let mut r = 0;

    while i < input.len() {
        i += 2; // skip "p="
        robots[r].0 = read_unsigned!(input, i);
        i += 1; // skip ","
        robots[r].1 = read_unsigned!(input, i);
        i += 3; // skip " v="
        robots[r].2 = read_signed!(input, i);
        i += 1; // skip ","
        robots[r].3 = read_signed!(input, i);
        i += 1; // skip "\n"
        r += 1;
    }

    let mut seconds = 0;
    let mut columns = [0; MS];
    let mut lines = [0; NS];
    let mut vertical_pattern_second = 0;
    let mut horizontal_pattern_second = 0;

    loop {
        seconds += 1;

        for (px, py, vx, vy) in robots.iter() {
            let x = ((px + seconds * vx) % M + M) % M;
            let y = ((py + seconds * vy) % N + N) % N;

            *columns.get_unchecked_mut(x as usize) += 1;
            *lines.get_unchecked_mut(y as usize) += 1;
        }

        if vertical_pattern_second == 0 && is_dispersion_big(&columns, M) {
            vertical_pattern_second = seconds;

            if horizontal_pattern_second != 0 {
                break;
            }
        }

        if horizontal_pattern_second == 0 && is_dispersion_big(&lines, N) {
            horizontal_pattern_second = seconds;

            if vertical_pattern_second != 0 {
                break;
            }
        }

        columns.fill(0);
        lines.fill(0);
    }

    let mut seconds = horizontal_pattern_second;

    loop {
        if (seconds - vertical_pattern_second) % M == 0 {
            return seconds;
        }

        seconds += N;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day14_part1() {
        let prod_input = read_to_string("./inputs/14.txt").unwrap();
        let prod_output = read_to_string("./outputs/14p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day14_part2() {
        let prod_input = read_to_string("./inputs/14.txt").unwrap();
        let prod_output = read_to_string("./outputs/14p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
