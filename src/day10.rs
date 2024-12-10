use std::collections::HashSet;

const POSSIBLE_MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn part1(input: &str) -> usize {
    let lines: &[u8] = input.trim_end().as_bytes();

    let mut m = 0;
    for &c in lines {
        if c == b'\n' {
            break;
        }
        m += 1;
    }

    let n = lines.len() as i32 / m;
    let mut pathes: HashSet<i32> = HashSet::new();

    fn r(
        start_x: i32,
        start_y: i32,
        m: i32,
        n: i32,
        lines: &[u8],
        pathes: &mut HashSet<i32>,
        x: i32,
        y: i32,
        pos: u8,
    ) {
        let next_pos = pos + 1;

        for &(dx, dy) in POSSIBLE_MOVES.iter() {
            let next_x = x + dx;
            let next_y = y + dy;

            if next_x >= 0
                && next_y >= 0
                && next_y < n
                && next_x < m
                && lines[get_index(next_x, next_y, m)] == next_pos
            {
                if next_pos == b'9' {
                    pathes.insert((start_y * m + start_x) + (next_y * m + next_x) * 10000);
                } else {
                    r(
                        start_x, start_y, m, n, lines, pathes, next_x, next_y, next_pos,
                    );
                }
            }
        }
    }

    for y in 0..n {
        for x in 0..m {
            if lines[get_index(x, y, m)] == b'0' {
                r(x, y, m, n, &lines, &mut pathes, x, y, b'0');
            }
        }
    }

    pathes.len()
}

fn get_index(x: i32, y: i32, m: i32) -> usize {
    // also newlines are counted
    (y * (m + 1) + x) as usize
}

pub fn part2(input: &str) -> usize {
    let lines: Vec<&[u8]> = input
        .trim_end()
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    let m = lines[0].len() as i32;
    let n = lines.len() as i32;

    fn r(
        start_x: i32,
        start_y: i32,
        m: i32,
        n: i32,
        lines: &Vec<&[u8]>,
        x: i32,
        y: i32,
        pos: u8,
    ) -> usize {
        let next_pos = pos + 1;
        let mut result = 0;

        for &(dx, dy) in POSSIBLE_MOVES.iter() {
            let next_x = x + dx;
            let next_y = y + dy;

            if next_x >= 0
                && next_y >= 0
                && next_y < n
                && next_x < m
                && lines[next_y as usize][next_x as usize] == next_pos
            {
                if next_pos == b'9' {
                    result += 1;
                } else {
                    result += r(start_x, start_y, m, n, lines, next_x, next_y, next_pos);
                }
            }
        }

        result
    }

    let mut result = 0;

    for y in 0..n {
        for x in 0..m {
            if lines[y as usize][x as usize] == b'0' {
                result += r(x, y, m, n, &lines, x, y, b'0');
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_day10_part1() {
        assert_eq!(part1(TEST_INPUT), 36);
        let prod_input = read_to_string("./inputs/10.txt").unwrap();
        let prod_output = read_to_string("./outputs/10p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day10_part2() {
        assert_eq!(part2(TEST_INPUT), 81);
        let prod_input = read_to_string("./inputs/10.txt").unwrap();
        let prod_output = read_to_string("./outputs/10p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
