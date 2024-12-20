const WIDTH: usize = 141;
const HEIGHT: usize = 141;
const LINE_LENGTH: usize = WIDTH + 1;
const GRID_SIZE: usize = LINE_LENGTH * HEIGHT;
const SAVED_TIME_LIMIT: usize = 100;

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

fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
}

unsafe fn solve(input: &str, max_cheating_time: usize) -> usize {
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

        let pos_x = pos % LINE_LENGTH;
        let pos_y = pos / LINE_LENGTH;

        let x_from = if pos_x > max_cheating_time + 1 {
            pos_x - max_cheating_time
        } else {
            1
        };
        let x_to = (WIDTH - 2).min(pos_x + max_cheating_time);
        let y_from = if pos_y > max_cheating_time + 1 {
            pos_y - max_cheating_time
        } else {
            1
        };
        let y_to = (HEIGHT - 2).min(pos_y + max_cheating_time);

        for ny in y_from..=y_to {
            for nx in x_from..=x_to {
                let time_after_cheating = *distances.get_unchecked(ny * LINE_LENGTH + nx);

                if time_after_cheating >= time_to_end {
                    continue;
                }

                let cheating_time = distance(nx, ny, pos_x, pos_y);

                if cheating_time > max_cheating_time {
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
    unsafe { solve(input, 2) }
}

pub fn part2(input: &str) -> usize {
    unsafe { solve(input, 20) }
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
