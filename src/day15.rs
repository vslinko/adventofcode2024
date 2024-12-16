const WIDTH: usize = 50;
const HEIGHT: usize = 50;
const LINE_WIDTH: usize = WIDTH + 1;
const GRID_LENGTH: usize = HEIGHT * LINE_WIDTH - 1;
const MOVES_START_INDEX: usize = GRID_LENGTH + 2;

const WIDTH2: usize = 100;
const HEIGHT2: usize = 50;

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

fn get_index(x: usize, y: usize) -> usize {
    y * LINE_WIDTH + x
}

unsafe fn inner1(input: &str) -> usize {
    let mut grid = input[0..GRID_LENGTH].as_bytes().to_vec();
    let moves = input[MOVES_START_INDEX..].as_bytes();

    let robot_pos = grid
        .iter()
        .position(|&cell| cell == b'@')
        .unwrap_unchecked();
    let mut robot_y = robot_pos / LINE_WIDTH;
    let mut robot_x = robot_pos % LINE_WIDTH;
    *grid.get_unchecked_mut(robot_pos) = b'.';

    for &movement in moves.iter() {
        match movement {
            b'^' => {
                let up_pos = get_index(robot_x, robot_y.wrapping_sub(1));

                match grid.get_unchecked(up_pos) {
                    b'.' => robot_y -= 1,
                    b'O' => {
                        for y in (0..robot_y - 1).rev() {
                            let pos = get_index(robot_x, y);

                            match grid.get_unchecked(pos) {
                                b'#' => break,
                                b'.' => {
                                    *grid.get_unchecked_mut(pos) = b'O';
                                    *grid.get_unchecked_mut(up_pos) = b'.';
                                    robot_y -= 1;
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            b'v' => {
                let down_pos = get_index(robot_x, robot_y + 1);

                match grid.get_unchecked(down_pos) {
                    b'.' => robot_y += 1,
                    b'O' => {
                        for y in (robot_y + 2)..HEIGHT {
                            let pos = get_index(robot_x, y);

                            match grid.get_unchecked(pos) {
                                b'#' => break,
                                b'.' => {
                                    *grid.get_unchecked_mut(pos) = b'O';
                                    *grid.get_unchecked_mut(down_pos) = b'.';
                                    robot_y += 1;
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            b'<' => {
                let left_pos = get_index(robot_x.wrapping_sub(1), robot_y);

                match grid.get_unchecked(left_pos) {
                    b'.' => robot_x -= 1,
                    b'O' => {
                        for x in (0..robot_x - 1).rev() {
                            let pos = get_index(x, robot_y);

                            match grid.get_unchecked(pos) {
                                b'#' => break,
                                b'.' => {
                                    *grid.get_unchecked_mut(pos) = b'O';
                                    *grid.get_unchecked_mut(left_pos) = b'.';
                                    robot_x -= 1;
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            b'>' => {
                let right_pos = get_index(robot_x + 1, robot_y);

                match grid.get_unchecked(right_pos) {
                    b'.' => robot_x += 1,
                    b'O' => {
                        for x in (robot_x + 2)..WIDTH {
                            let pos = get_index(x, robot_y);

                            match grid.get_unchecked(pos) {
                                b'#' => break,
                                b'.' => {
                                    *grid.get_unchecked_mut(pos) = b'O';
                                    *grid.get_unchecked_mut(right_pos) = b'.';
                                    robot_x += 1;
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let mut solution = 0;
    for y in 0..HEIGHT {
        let row_value = y * 100;
        for x in 0..WIDTH {
            if *grid.get_unchecked(get_index(x, y)) == b'O' {
                solution += row_value + x;
            }
        }
    }

    solution
}

fn get_index2(x: usize, y: usize) -> usize {
    y * WIDTH2 + x
}

pub fn part2(input: &str) -> usize {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> usize {
    let moves = input[MOVES_START_INDEX..].as_bytes();

    let mut grid = Vec::with_capacity(WIDTH2 * HEIGHT2);
    for &c in input[0..GRID_LENGTH].as_bytes() {
        match c {
            b'@' => {
                grid.extend_from_slice(&[b'@', b'.']);
            }
            b'O' => {
                grid.extend_from_slice(&[b'[', b']']);
            }
            b'#' => {
                grid.extend_from_slice(&[b'#', b'#']);
            }
            b'.' => {
                grid.extend_from_slice(&[b'.', b'.']);
            }
            _ => {}
        }
    }

    let mut robot_pos = grid
        .iter()
        .position(|&cell| cell == b'@')
        .unwrap_unchecked();
    *grid.get_unchecked_mut(robot_pos) = b'.';

    #[inline(always)]
    const fn is_box(cell: u8) -> bool {
        cell == b'[' || cell == b']'
    }

    let mut boxes_to_move = Vec::with_capacity(100);

    #[inline(always)]
    fn recursive_move_up(grid: &[u8], boxes_to_move: &mut Vec<usize>, pos: usize) -> bool {
        let pos = if grid[pos] == b']' { pos - 1 } else { pos };
        let next_pos = pos - WIDTH2;
        let next_pos1 = next_pos + 1;

        if grid[next_pos] == b'#' || grid[next_pos1] == b'#' {
            return false;
        }

        if is_box(grid[next_pos]) && !recursive_move_up(grid, boxes_to_move, next_pos) {
            return false;
        }

        if grid[next_pos1] == b'[' && !recursive_move_up(grid, boxes_to_move, next_pos1) {
            return false;
        }

        boxes_to_move.push(pos);
        boxes_to_move.push(pos + 1);
        true
    }

    #[inline(always)]
    fn recursive_move_down(grid: &[u8], boxes_to_move: &mut Vec<usize>, pos: usize) -> bool {
        let pos = if grid[pos] == b']' { pos - 1 } else { pos };
        let next_pos = pos + WIDTH2;
        let next_pos1 = next_pos + 1;

        if grid[next_pos] == b'#' || grid[next_pos1] == b'#' {
            return false;
        }

        if is_box(grid[next_pos]) && !recursive_move_down(grid, boxes_to_move, next_pos) {
            return false;
        }

        if grid[next_pos1] == b'[' && !recursive_move_down(grid, boxes_to_move, next_pos1) {
            return false;
        }

        boxes_to_move.push(pos);
        boxes_to_move.push(pos + 1);
        true
    }

    for &movement in moves {
        match movement {
            b'^' => {
                let next_pos = robot_pos - WIDTH2;
                match grid[next_pos] {
                    b'.' => robot_pos = next_pos,
                    b'[' | b']' => {
                        boxes_to_move.clear();
                        if recursive_move_up(&grid, &mut boxes_to_move, next_pos) {
                            boxes_to_move.sort_unstable();
                            boxes_to_move.dedup();
                            for pos in boxes_to_move.iter() {
                                grid[pos - WIDTH2] = grid[*pos];
                                grid[*pos] = b'.';
                            }
                            robot_pos = next_pos;
                        }
                    }
                    _ => {}
                }
            }
            b'v' => {
                let next_pos = robot_pos + WIDTH2;
                match grid[next_pos] {
                    b'.' => robot_pos = next_pos,
                    b'[' | b']' => {
                        boxes_to_move.clear();
                        if recursive_move_down(&grid, &mut boxes_to_move, next_pos) {
                            boxes_to_move.sort_unstable();
                            boxes_to_move.dedup();
                            for &pos in boxes_to_move.iter().rev() {
                                grid[pos + WIDTH2] = grid[pos];
                                grid[pos] = b'.';
                            }
                            robot_pos = next_pos;
                        }
                    }
                    _ => {}
                }
            }
            b'<' => {
                let next_pos = robot_pos - 1;
                if grid[next_pos] == b'.' {
                    robot_pos = next_pos;
                } else if is_box(grid[next_pos]) {
                    let row_start = robot_pos - robot_pos % WIDTH2;
                    for pos in (row_start..next_pos).rev() {
                        if grid[pos] == b'#' {
                            break;
                        }
                        if grid[pos] == b'.' {
                            for curr_pos in pos..robot_pos {
                                grid[curr_pos] = grid[curr_pos + 1];
                            }
                            grid[next_pos] = b'.';
                            robot_pos = next_pos;
                            break;
                        }
                    }
                }
            }
            b'>' => {
                let next_pos = robot_pos + 1;
                if grid[next_pos] == b'.' {
                    robot_pos = next_pos;
                } else if is_box(grid[next_pos]) {
                    let row_end = robot_pos - robot_pos % WIDTH2 + WIDTH2;
                    for pos in robot_pos + 2..row_end {
                        match grid[pos] {
                            b'#' => break,
                            b'.' => {
                                for curr_pos in (robot_pos + 1..=pos).rev() {
                                    grid[curr_pos] = grid[curr_pos - 1];
                                }
                                grid[next_pos] = b'.';
                                robot_pos = next_pos;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let mut solution = 0;
    for y in 0..HEIGHT2 {
        let row_value = y * 100;
        for x in 0..WIDTH2 {
            if grid[get_index2(x, y)] == b'[' {
                solution += row_value + x;
            }
        }
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day15_part1() {
        let prod_input = read_to_string("./inputs/15.txt").unwrap();
        let prod_output = read_to_string("./outputs/15p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day15_part2() {
        let prod_input = read_to_string("./inputs/15.txt").unwrap();
        let prod_output = read_to_string("./outputs/15p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
