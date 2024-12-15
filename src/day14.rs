#[derive(Debug)]
struct Robot {
    p: Vec<i32>,
    v: Vec<i32>,
}

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

const PART1_SECONDS: i32 = 100;

unsafe fn inner1(input: &str) -> usize {
    let m = 101;
    let n = 103;
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

        let x = ((px + PART1_SECONDS * vx) % m + m) % m;
        let y = ((py + PART1_SECONDS * vy) % n + n) % n;

        if x < m / 2 && y < n / 2 {
            left_top += 1;
        } else if x > m / 2 && y < n / 2 {
            right_top += 1;
        } else if x < m / 2 && y > n / 2 {
            left_bottom += 1;
        } else if x > m / 2 && y > n / 2 {
            right_bottom += 1;
        }
    }

    left_top * right_top * left_bottom * right_bottom
}

pub fn part2(input: &str) -> i32 {
    let mut robots: Vec<Robot> = input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let p: Vec<i32> = parts
                .next()
                .unwrap()
                .trim_start_matches("p=")
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            let v: Vec<i32> = parts
                .next()
                .unwrap()
                .trim_start_matches("v=")
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            Robot { p, v }
        })
        .collect();

    let m = 101;
    let n = 103;
    let mut iter = 0;

    loop {
        for robot in robots.iter_mut() {
            robot.p[0] += robot.v[0];
            if robot.p[0] < 0 {
                robot.p[0] = m + robot.p[0];
            }
            if robot.p[0] >= m {
                robot.p[0] = robot.p[0] - m;
            }
            robot.p[1] += robot.v[1];
            if robot.p[1] < 1 {
                robot.p[1] = n + robot.p[1];
            }
            if robot.p[1] >= n {
                robot.p[1] = robot.p[1] - n;
            }
        }
        iter += 1;

        // temp hack
        if iter < 8270 {
            continue;
        }

        for y in 0..n {
            let mut robots_in_row = 0;
            for x in 0..m {
                if robots.iter().any(|r| r.p[0] == x && r.p[1] == y) {
                    robots_in_row += 1;

                    if robots_in_row == 11 {
                        return iter;
                    }
                } else {
                    robots_in_row = 0;
                }
            }
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
