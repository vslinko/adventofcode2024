use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let mut parts = input.trim().split("\n\n");
    let grid_str = parts.next().unwrap();
    let moves = parts.next().unwrap().trim().chars().collect::<Vec<char>>();

    let mut grid: Vec<Vec<char>> = grid_str
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let m = grid[0].len();
    let n = grid.len();

    let mut robot_y = 0;
    let mut robot_x = 0;
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '@' {
                robot_y = y;
                robot_x = x;
                break 'outer;
            }
        }
    }
    grid[robot_y][robot_x] = '.';

    for &movement in moves.iter() {
        match movement {
            '^' => {
                if grid[robot_y - 1][robot_x] == '.' {
                    robot_y -= 1;
                } else if grid[robot_y - 1][robot_x] == 'O' {
                    let mut next_dot_index = None;
                    for y in (0..robot_y - 1).rev() {
                        if grid[y][robot_x] == '#' {
                            break;
                        }
                        if grid[y][robot_x] == '.' {
                            next_dot_index = Some(y);
                            break;
                        }
                    }

                    if let Some(y) = next_dot_index {
                        grid[y][robot_x] = 'O';
                        grid[robot_y - 1][robot_x] = '.';
                        robot_y -= 1;
                    }
                }
            }
            'v' => {
                if grid[robot_y + 1][robot_x] == '.' {
                    robot_y += 1;
                } else if grid[robot_y + 1][robot_x] == 'O' {
                    let mut next_dot_index = None;
                    for y in robot_y + 2..n {
                        if grid[y][robot_x] == '#' {
                            break;
                        }
                        if grid[y][robot_x] == '.' {
                            next_dot_index = Some(y);
                            break;
                        }
                    }

                    if let Some(y) = next_dot_index {
                        grid[y][robot_x] = 'O';
                        grid[robot_y + 1][robot_x] = '.';
                        robot_y += 1;
                    }
                }
            }
            '<' => {
                if grid[robot_y][robot_x - 1] == '.' {
                    robot_x -= 1;
                } else if grid[robot_y][robot_x - 1] == 'O' {
                    let mut next_dot_index = None;
                    for x in (0..robot_x - 1).rev() {
                        if grid[robot_y][x] == '#' {
                            break;
                        }
                        if grid[robot_y][x] == '.' {
                            next_dot_index = Some(x);
                            break;
                        }
                    }

                    if let Some(x) = next_dot_index {
                        grid[robot_y][x] = 'O';
                        grid[robot_y][robot_x - 1] = '.';
                        robot_x -= 1;
                    }
                }
            }
            '>' => {
                if grid[robot_y][robot_x + 1] == '.' {
                    robot_x += 1;
                } else if grid[robot_y][robot_x + 1] == 'O' {
                    let mut next_dot_index = None;
                    for x in robot_x + 2..m {
                        if grid[robot_y][x] == '#' {
                            break;
                        }
                        if grid[robot_y][x] == '.' {
                            next_dot_index = Some(x);
                            break;
                        }
                    }

                    if let Some(x) = next_dot_index {
                        grid[robot_y][x] = 'O';
                        grid[robot_y][robot_x + 1] = '.';
                        robot_x += 1;
                    }
                }
            }
            '\n' => {}
            _ => panic!("Unknown move: {}", movement),
        }
    }

    let mut solution = 0;
    for y in 0..n {
        for x in 0..m {
            if grid[y][x] == 'O' {
                solution += y * 100 + x;
            }
        }
    }

    solution
}

pub fn part2(input: &str) -> i32 {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> i32 {
    let mut parts = input.trim().split("\n\n");
    let grid_str = parts.next().unwrap();
    let moves = parts.next().unwrap().trim().chars().collect::<Vec<char>>();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in grid_str.lines() {
        let mut new_row = Vec::new();
        for c in line.chars() {
            match c {
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                }
                _ => panic!("Unknown cell: {}", c),
            }
        }
        grid.push(new_row);
    }

    let m = grid[0].len();
    let n = grid.len();

    let mut robot_y = 0;
    let mut robot_x = 0;
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '@' {
                robot_y = y;
                robot_x = x;
                break 'outer;
            }
        }
    }
    grid[robot_y][robot_x] = '.';

    fn is_box(cell: char) -> bool {
        cell == '[' || cell == ']'
    }

    fn recursive_move_up(
        grid: &Vec<Vec<char>>,
        boxes_to_move: &mut HashSet<(usize, usize)>,
        x: usize,
        y: usize,
    ) -> bool {
        let mut curr_x = x;
        if grid[y][curr_x] == ']' {
            curr_x -= 1;
        }

        assert_eq!(grid[y][curr_x], '[');

        if grid[y - 1][curr_x] == '#' || grid[y - 1][curr_x + 1] == '#' {
            return false;
        }

        if is_box(grid[y - 1][curr_x]) && !recursive_move_up(grid, boxes_to_move, curr_x, y - 1) {
            return false;
        }

        if grid[y - 1][curr_x + 1] == '['
            && !recursive_move_up(grid, boxes_to_move, curr_x + 1, y - 1)
        {
            return false;
        }

        boxes_to_move.insert((curr_x, y));
        boxes_to_move.insert((curr_x + 1, y));

        true
    }

    fn recursive_move_down(
        grid: &Vec<Vec<char>>,
        boxes_to_move: &mut HashSet<(usize, usize)>,
        x: usize,
        y: usize,
    ) -> bool {
        let mut curr_x = x;
        if grid[y][curr_x] == ']' {
            curr_x -= 1;
        }

        assert_eq!(grid[y][curr_x], '[');

        if grid[y + 1][curr_x] == '#' || grid[y + 1][curr_x + 1] == '#' {
            return false;
        }

        if is_box(grid[y + 1][curr_x]) && !recursive_move_down(grid, boxes_to_move, curr_x, y + 1) {
            return false;
        }

        if grid[y + 1][curr_x + 1] == '['
            && !recursive_move_down(grid, boxes_to_move, curr_x + 1, y + 1)
        {
            return false;
        }

        boxes_to_move.insert((curr_x, y));
        boxes_to_move.insert((curr_x + 1, y));

        true
    }

    for &movement in moves.iter() {
        match movement {
            '^' => {
                if grid[robot_y - 1][robot_x] == '.' {
                    robot_y -= 1;
                } else if is_box(grid[robot_y - 1][robot_x]) {
                    let mut boxes_to_move = HashSet::new();

                    if !recursive_move_up(&grid, &mut boxes_to_move, robot_x, robot_y - 1) {
                        continue;
                    }

                    let mut moves = boxes_to_move.iter().collect::<Vec<_>>();
                    moves.sort_unstable_by_key(|&(_, y)| y);

                    for &(x, y) in moves {
                        grid[y - 1][x] = grid[y][x];
                        grid[y][x] = '.';
                    }
                    robot_y -= 1;
                }
            }
            'v' => {
                if grid[robot_y + 1][robot_x] == '.' {
                    robot_y += 1;
                } else if is_box(grid[robot_y + 1][robot_x]) {
                    let mut boxes_to_move = HashSet::new();

                    if !recursive_move_down(&grid, &mut boxes_to_move, robot_x, robot_y + 1) {
                        continue;
                    }

                    let mut moves = boxes_to_move.iter().collect::<Vec<_>>();
                    moves.sort_by_key(|&(_, y)| std::cmp::Reverse(y));

                    for &(x, y) in moves {
                        grid[y + 1][x] = grid[y][x];
                        grid[y][x] = '.';
                    }
                    robot_y += 1;
                }
            }
            '<' => {
                if grid[robot_y][robot_x - 1] == '.' {
                    robot_x -= 1;
                } else if is_box(grid[robot_y][robot_x - 1]) {
                    let mut next_dot_index = None;
                    for x in (0..robot_x - 1).rev() {
                        if grid[robot_y][x] == '#' {
                            break;
                        }
                        if grid[robot_y][x] == '.' {
                            next_dot_index = Some(x);
                            break;
                        }
                    }

                    if let Some(x) = next_dot_index {
                        for curr_x in x..robot_x {
                            grid[robot_y][curr_x] = grid[robot_y][curr_x + 1];
                        }
                        grid[robot_y][robot_x - 1] = '.';
                        robot_x -= 1;
                    }
                }
            }
            '>' => {
                if grid[robot_y][robot_x + 1] == '.' {
                    robot_x += 1;
                } else if is_box(grid[robot_y][robot_x + 1]) {
                    let mut next_dot_index = None;
                    for x in robot_x + 2..m {
                        if grid[robot_y][x] == '#' {
                            break;
                        }
                        if grid[robot_y][x] == '.' {
                            next_dot_index = Some(x);
                            break;
                        }
                    }

                    if let Some(x) = next_dot_index {
                        for curr_x in (robot_x + 1..=x).rev() {
                            grid[robot_y][curr_x] = grid[robot_y][curr_x - 1];
                        }
                        grid[robot_y][robot_x + 1] = '.';
                        robot_x += 1;
                    }
                }
            }
            '\n' => {}
            _ => panic!("Unknown move: {}", movement),
        }
    }

    let mut solution = 0;
    for y in 0..n {
        for x in 0..m {
            if grid[y][x] == '[' {
                solution += (y * 100 + x) as i32;
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
