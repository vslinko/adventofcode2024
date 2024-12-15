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

fn calculate_robots_group_size(visited: &mut [bool], result: &[bool], x: i32, y: i32) -> i32 {
    let index = y * M + x;
    let i = index as usize;

    if index < 0 || index >= GRID_SIZE || visited[i] || !result[i] {
        return 0;
    }

    visited[i] = true;

    return 1
        + calculate_robots_group_size(visited, result, x, y + 1)
        + calculate_robots_group_size(visited, result, x, y - 1)
        + calculate_robots_group_size(visited, result, x + 1, y)
        + calculate_robots_group_size(visited, result, x - 1, y);
}

fn find_max_robots_group(robots: &[(i32, i32, i32, i32)], seconds: i32) -> i32 {
    let mut grid = vec![false; GRID_SIZE as usize];
    let mut visited = vec![false; GRID_SIZE as usize];
    let mut maximum = 0;

    for (px, py, vx, vy) in robots.iter() {
        let x = ((px + seconds * vx) % M + M) % M;
        let y = ((py + seconds * vy) % N + N) % N;
        grid[(y * M + x) as usize] = true;
    }

    for i in 0..GRID_SIZE {
        if grid[i as usize] {
            let x = i % M;
            let y = i / M;
            maximum = maximum.max(calculate_robots_group_size(&mut visited, &grid, x, y));
        }
    }

    maximum
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

    let mut seconds = 1;

    while find_max_robots_group(&robots, seconds) < 62 {
        seconds += 1;
    }

    seconds
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
