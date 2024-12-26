pub fn part1(input: &str) -> i32 {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let m = grid[0].len();
    let n = grid.len();
    let max_m = m - 3;
    let max_n = n - 3;

    let mut xmases = 0;

    // horizontal
    for y in 0..n {
        for x in 0..max_m {
            let substring = &grid[y][x..x + 4];

            if is_xmas(substring) {
                xmases += 1;
            }
        }
    }

    // vertical
    for y in 0..max_n {
        for x in 0..m {
            let substring = [grid[y][x], grid[y + 1][x], grid[y + 2][x], grid[y + 3][x]];

            if is_xmas(&substring) {
                xmases += 1;
            }
        }
    }

    // diagonal l->r
    for y in 0..max_n {
        for x in 0..max_m {
            let subgrid = [
                grid[y][x],
                grid[y + 1][x + 1],
                grid[y + 2][x + 2],
                grid[y + 3][x + 3],
            ];

            if is_xmas(&subgrid) {
                xmases += 1;
            }
        }
    }

    // diagonal l<-r
    for y in 0..max_n {
        for x in 3..m {
            let subgrid = [
                grid[y][x],
                grid[y + 1][x - 1],
                grid[y + 2][x - 2],
                grid[y + 3][x - 3],
            ];

            if is_xmas(&subgrid) {
                xmases += 1;
            }
        }
    }

    xmases
}

pub fn part2(input: &str) -> i32 {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let m = grid[0].len();
    let n = grid.len();

    let mut mases = 0;

    for y in 0..n - 2 {
        for x in 0..m - 2 {
            let diagonal_left_to_right = [grid[y][x], grid[y + 1][x + 1], grid[y + 2][x + 2]];
            let diagonal_right_to_left = [grid[y][x + 2], grid[y + 1][x + 1], grid[y + 2][x]];

            if is_mas(&diagonal_left_to_right) && is_mas(&diagonal_right_to_left) {
                mases += 1;
            }
        }
    }

    mases
}

fn is_xmas(substring: &[u8]) -> bool {
    substring == [b'X', b'M', b'A', b'S'] || substring == [b'S', b'A', b'M', b'X']
}

fn is_mas(substring: &[u8]) -> bool {
    substring == [b'M', b'A', b'S'] || substring == [b'S', b'A', b'M']
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day4_part1() {
        let prod_input = read_to_string("./inputs/4.txt").unwrap();
        let prod_output = read_to_string("./outputs/4p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day4_part2() {
        let prod_input = read_to_string("./inputs/4.txt").unwrap();
        let prod_output = read_to_string("./outputs/4p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
