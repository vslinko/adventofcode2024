use rayon::iter::*;

const WIDTH: usize = 141;
const HEIGHT: usize = 141;
const LINE_LENGTH: usize = WIDTH + 1;
const GRID_SIZE: usize = LINE_LENGTH * HEIGHT;
const SAVED_TIME_LIMIT: usize = 100;
const MIN_X: isize = 1;
const MAX_X: isize = (WIDTH - 2) as isize;
const MIN_Y: isize = 1;
const MAX_Y: isize = (HEIGHT - 2) as isize;
const EMPTY_CELL: usize = usize::MAX / 2;

macro_rules! generate_jumps {
    ($size:expr, $max_dist:expr) => {{
        let mut lut = [(0, 0, 0); $size];
        let mut i = 0;

        let mut y: isize = -$max_dist;
        while y <= $max_dist {
            let mut x: isize = -$max_dist;
            while x <= $max_dist {
                let d = (x.abs() + y.abs()) as usize;
                if d > 0 && d <= $max_dist {
                    lut[i] = (x, y, d);
                    i += 1;
                }
                x += 1;
            }
            y += 1;
        }

        lut
    }};
}

const CHEATING_JUMPS1: [(isize, isize, usize); 12] = generate_jumps!(12, 2);
const CHEATING_JUMPS2: [(isize, isize, usize); 840] = generate_jumps!(840, 20);

macro_rules! parse {
    ($input:expr, $distances:expr) => {{
        let grid = $input.as_bytes();
        let start = grid.iter().position(|&c| c == b'S').unwrap_unchecked();
        let end = grid.iter().position(|&c| c == b'E').unwrap_unchecked();

        calc_distances(&grid, start, end, 0, $distances);

        (start, end, *$distances.get_unchecked(start))
    }};
}

macro_rules! iterate_path {
    ($distances:expr, $start:expr, $end:expr, |$pos:ident| $block:block) => {
        let mut $pos = $start;

        while $pos != $end {
            $block

            let next_pos_expected_time = *$distances.get_unchecked($pos) - 1;

            $pos = if *$distances.get_unchecked($pos - 1) == next_pos_expected_time {
                $pos - 1
            } else if *$distances.get_unchecked($pos + 1) == next_pos_expected_time {
                $pos + 1
            } else if *$distances.get_unchecked($pos - LINE_LENGTH) == next_pos_expected_time {
                $pos - LINE_LENGTH
            } else if *$distances.get_unchecked($pos + LINE_LENGTH) == next_pos_expected_time {
                $pos + LINE_LENGTH
            } else {
                unreachable!()
            };
        }
    };
}

macro_rules! calculate_cheating_jumps {
    ($distances:expr, $initial_total_time:expr, $pos:expr, $jumps:expr) => {{
        let time_before_cheating = $initial_total_time - *$distances.get_unchecked($pos);
        let pos_x = ($pos % LINE_LENGTH) as isize;
        let pos_y = ($pos / LINE_LENGTH) as isize;

        $jumps
            .iter()
            .filter(|(dx, dy, cheating_time)| {
                let cheat_x = pos_x + dx;
                let cheat_y = pos_y + dy;

                if cheat_x < MIN_X || cheat_x > MAX_X || cheat_y < MIN_Y || cheat_y > MAX_Y {
                    return false;
                }

                let time_after_cheating =
                    *$distances.get_unchecked(cheat_y as usize * LINE_LENGTH + cheat_x as usize);

                let total_time = time_before_cheating + cheating_time + time_after_cheating;

                if total_time > $initial_total_time {
                    return false;
                }

                let saved_time = $initial_total_time - total_time;

                if saved_time < SAVED_TIME_LIMIT {
                    return false;
                }

                true
            })
            .count()
    }};
}

unsafe fn calc_distances(
    grid: &[u8],
    end: usize,
    pos: usize,
    dist: usize,
    distances: &mut [usize; GRID_SIZE],
) {
    if *distances.get_unchecked(pos) != EMPTY_CELL || *distances.get_unchecked(end) != EMPTY_CELL {
        return;
    }

    *distances.get_unchecked_mut(pos) = dist;

    let next_dist = dist + 1;

    macro_rules! calc_next {
        ($next_pos:expr) => {
            let next_pos = $next_pos;

            if *grid.get_unchecked(next_pos) != b'#' {
                calc_distances(grid, end, next_pos, next_dist, distances);
            }
        };
    }

    calc_next!(pos - 1);
    calc_next!(pos + 1);
    calc_next!(pos - LINE_LENGTH);
    calc_next!(pos + LINE_LENGTH);
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let mut distances = [EMPTY_CELL; GRID_SIZE];
    let (start, end, initial_total_time) = parse!(input, &mut distances);

    let mut result = 0;

    iterate_path!(distances, start, end, |pos| {
        result += calculate_cheating_jumps!(distances, initial_total_time, pos, CHEATING_JUMPS1);
    });

    result
}

pub fn part2(input: &str) -> usize {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> usize {
    let mut distances = [EMPTY_CELL; GRID_SIZE];
    let (start, end, initial_total_time) = parse!(input, &mut distances);

    let mut path = Vec::with_capacity(initial_total_time);
    iterate_path!(distances, start, end, |pos| {
        path.push(pos);
    });

    path.par_iter()
        .map(|&pos| calculate_cheating_jumps!(distances, initial_total_time, pos, CHEATING_JUMPS2))
        .sum()
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
