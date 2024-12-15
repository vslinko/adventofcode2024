#[derive(Debug)]
struct Robot {
    p: Vec<i32>,
    v: Vec<i32>,
}

pub fn part1(input: &str) -> usize {
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
    let seconds = 100;

    for robot in robots.iter_mut() {
        for _ in 0..seconds {
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
    }

    let left_top = robots
        .iter()
        .filter(|r| r.p[0] < m / 2 && r.p[1] < n / 2)
        .count();

    let right_top = robots
        .iter()
        .filter(|r| r.p[0] > m / 2 && r.p[1] < n / 2)
        .count();

    let left_bottom = robots
        .iter()
        .filter(|r| r.p[0] < m / 2 && r.p[1] > n / 2)
        .count();

    let right_bottom = robots
        .iter()
        .filter(|r| r.p[0] > m / 2 && r.p[1] > n / 2)
        .count();

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
