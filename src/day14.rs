const M: i32 = 101;
const N: i32 = 103;
const GRID_SIZE: i32 = M * N;
const HALF_M: i32 = M / 2;
const HALF_N: i32 = N / 2;
const PART1_SECONDS: i32 = 100;
const PART2_ROBOTS: usize = 500;

macro_rules! read {
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
        let px = read!(input, i);
        i += 1; // skip ","
        let py = read!(input, i);
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

#[derive(Debug)]
enum GridPatten {
    EasterEgg,
    Horisontal,
    Vertical,
    None,
}

fn has_pattern(robots: &[(i32, i32, i32, i32)], seconds: i32) -> GridPatten {
    let mut columns = [0; M as usize];
    let mut lines = [0; N as usize];

    for (px, py, vx, vy) in robots.iter() {
        let x = ((px + seconds * vx) % M + M) % M;
        let y = ((py + seconds * vy) % N + N) % N;
        columns[x as usize] += 1;
        lines[y as usize] += 1;
    }

    let has_vertical_pattern = fast_dispersion(&columns, M) > 250_000;
    let has_horisontal_pattern = fast_dispersion(&lines, N) > 250_000;

    match (has_vertical_pattern, has_horisontal_pattern) {
        (true, true) => GridPatten::EasterEgg,
        (true, false) => GridPatten::Vertical,
        (false, true) => GridPatten::Horisontal,
        (false, false) => GridPatten::None,
    }
}

#[allow(dead_code)]
fn print_grid(grid: &[bool; GRID_SIZE as usize]) {
    for y in 0..N {
        for x in 0..M {
            print!("{}", if grid[(y * M + x) as usize] { '#' } else { '.' });
        }
        println!();
    }
    println!();
}

fn fast_dispersion(arr: &[i32], n: i32) -> i32 {
    let mut sum = 0;
    let mut sum_sq = 0;

    for &count in arr {
        sum += count;
        sum_sq += count * count;
    }

    sum_sq * n - sum * sum
}

unsafe fn inner2(input: &str) -> i32 {
    let input = input.as_bytes();

    let mut robots = [(0, 0, 0, 0); PART2_ROBOTS];
    let mut i = 0;
    let mut r = 0;

    while i < input.len() {
        i += 2; // skip "p="
        robots[r].0 = read!(input, i);
        i += 1; // skip ","
        robots[r].1 = read!(input, i);
        i += 3; // skip " v="
        robots[r].2 = read_signed!(input, i);
        i += 1; // skip ","
        robots[r].3 = read_signed!(input, i);
        i += 1; // skip "\n"
        r += 1;
    }

    let mut seconds = 0;
    let mut speed = 1;

    loop {
        match has_pattern(&robots, seconds) {
            GridPatten::EasterEgg => {
                return seconds;
            }
            GridPatten::Horisontal => {
                speed = N;
            }
            GridPatten::Vertical => {
                speed = M;
            }
            GridPatten::None => {}
        }

        seconds += speed;

        if seconds > 8280 {
            return 0;
        }
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
