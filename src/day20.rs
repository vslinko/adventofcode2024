const WIDTH: usize = 141;
const HEIGHT: usize = 141;
const LINE_LENGTH: usize = WIDTH + 1;
const GRID_SIZE: usize = LINE_LENGTH * HEIGHT;
const SAVED_TIME_LIMIT: usize = 100;
const MIN_X: isize = 1;
const MAX_X: isize = (WIDTH - 2) as isize;
const MIN_Y: isize = 1;
const MAX_Y: isize = (HEIGHT - 2) as isize;

const CHEATING_JUMPS1: [(isize, isize, usize); 12] = {
    let mut lut = [(0, 0, 0); 12];
    let mut i = 0;

    let mut y = -2_isize;
    while y <= 2_isize {
        let mut x = -2_isize;
        while x <= 2_isize {
            let d = (x.abs() + y.abs()) as usize;
            if d > 0 && d <= 2 {
                lut[i] = (x, y, d);
                i += 1;
            }
            x += 1;
        }
        y += 1;
    }

    lut
};

const CHEATING_JUMPS2: [(isize, isize, usize); 840] = {
    let mut lut = [(0, 0, 0); 840];
    let mut i = 0;

    let mut y = -20_isize;
    while y <= 20_isize {
        let mut x = -20_isize;
        while x <= 20_isize {
            let d = (x.abs() + y.abs()) as usize;
            if d > 0 && d <= 20 {
                lut[i] = (x, y, d);
                i += 1;
            }
            x += 1;
        }
        y += 1;
    }

    lut
};

unsafe fn calc_distances(grid: &[u8], pos: usize, dist: usize, distances: &mut [usize; GRID_SIZE]) {
    if *distances.get_unchecked(pos) != usize::MAX {
        return;
    }

    *distances.get_unchecked_mut(pos) = dist;

    let next_dist = dist + 1;

    macro_rules! calc_next {
        ($next_pos:expr) => {
            let next_pos = $next_pos;

            if *grid.get_unchecked(next_pos) != b'#' {
                calc_distances(grid, next_pos, next_dist, distances);
            }
        };
    }

    calc_next!(pos - 1);
    calc_next!(pos + 1);
    calc_next!(pos - LINE_LENGTH);
    calc_next!(pos + LINE_LENGTH);
}

unsafe fn solve(input: &str, cheating_jumps: &[(isize, isize, usize)]) -> usize {
    let grid = input.as_bytes();
    let start = grid.iter().position(|&c| c == b'S').unwrap_unchecked();
    let end = grid.iter().position(|&c| c == b'E').unwrap_unchecked();

    let mut distances = [usize::MAX; GRID_SIZE];
    calc_distances(&grid, end, 0, &mut distances);

    let initial_total_time = *distances.get_unchecked(start);

    let mut pos = start;
    let mut result = 0;

    while pos != end {
        let time_to_end = *distances.get_unchecked(pos);
        let time_before_cheating = initial_total_time - time_to_end;

        let pos_x = (pos % LINE_LENGTH) as isize;
        let pos_y = (pos / LINE_LENGTH) as isize;

        for (dx, dy, cheating_time) in cheating_jumps.iter() {
            let cheat_x = pos_x + dx;
            let cheat_y = pos_y + dy;

            if cheat_x < MIN_X || cheat_x > MAX_X || cheat_y < MIN_Y || cheat_y > MAX_Y {
                continue;
            }

            let time_after_cheating =
                *distances.get_unchecked(cheat_y as usize * LINE_LENGTH + cheat_x as usize);

            if time_after_cheating >= time_to_end {
                continue;
            }

            let total_time = time_before_cheating + cheating_time + time_after_cheating;

            if total_time > initial_total_time {
                continue;
            }

            let saved_time = initial_total_time - total_time;

            if saved_time < SAVED_TIME_LIMIT {
                continue;
            }

            result += 1;
        }

        let next_pos_expected_time = time_to_end - 1;

        pos = if *distances.get_unchecked(pos - 1) == next_pos_expected_time {
            pos - 1
        } else if *distances.get_unchecked(pos + 1) == next_pos_expected_time {
            pos + 1
        } else if *distances.get_unchecked(pos - LINE_LENGTH) == next_pos_expected_time {
            pos - LINE_LENGTH
        } else if *distances.get_unchecked(pos + LINE_LENGTH) == next_pos_expected_time {
            pos + LINE_LENGTH
        } else {
            unreachable!()
        };
    }

    result
}

pub fn part1(input: &str) -> usize {
    unsafe { solve(input, &CHEATING_JUMPS1) }
}

pub fn part2(input: &str) -> usize {
    unsafe { solve(input, &CHEATING_JUMPS2) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day20_part1() {
        let prod_input = read_to_string("./inputs/20.txt").unwrap();
        let prod_output = read_to_string("./outputs/20p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day20_part2() {
        let prod_input = read_to_string("./inputs/20.txt").unwrap();
        let prod_output = read_to_string("./outputs/20p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
