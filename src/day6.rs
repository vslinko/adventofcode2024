const UP: u8 = 1;
const RIGHT: u8 = 2;
const DOWN: u8 = 4;
const LEFT: u8 = 8;

#[derive(PartialEq, Debug, Clone)]
enum Cell {
    Empty,
    Obstacle,
}

enum SimulationResult {
    Escaped(usize),
    Cycle,
}

type Position = (usize, usize, u8);
type Grid = Vec<Vec<Cell>>;

fn parse(input: &str) -> (Grid, Position) {
    let lines: Vec<&str> = input.trim().lines().collect();
    let m = lines[0].len();
    let n = lines.len();

    let mut grid = vec![vec![Cell::Empty; m]; n];
    let mut start_position = None;

    for (y, line) in lines.iter().enumerate() {
        assert_eq!(line.len(), m);

        for (x, char) in line.bytes().enumerate() {
            if char == b'#' {
                grid[y][x] = Cell::Obstacle;
            }

            start_position = match char {
                b'^' => Some((x, y, UP)),
                b'>' => Some((x, y, RIGHT)),
                b'v' => Some((x, y, DOWN)),
                b'<' => Some((x, y, LEFT)),
                _ => start_position,
            };
        }
    }

    (grid, start_position.expect("Start position not found"))
}

pub fn part1(input: &str) -> usize {
    let (grid, start_position) = parse(input);

    match simulate(&grid, &start_position) {
        SimulationResult::Escaped(visited) => visited,
        _ => panic!("Cycle found"),
    }
}

pub fn part2(input: &str) -> usize {
    let (grid, start_position) = parse(input);

    let m = grid[0].len();
    let n = grid.len();
    let (start_x, start_y, _) = start_position;

    let mut possible_places = 0;
    let mut grid = grid.clone();

    for y in 0..n {
        for x in 0..m {
            if (x == start_x && y == start_y) || grid[y][x] == Cell::Obstacle {
                continue;
            }

            grid[y][x] = Cell::Obstacle;

            possible_places += match simulate(&grid, &start_position) {
                SimulationResult::Cycle => 1,
                _ => 0,
            };

            grid[y][x] = Cell::Empty;
        }
    }

    possible_places
}

fn simulate(grid: &Grid, start_position: &Position) -> SimulationResult {
    let m = grid[0].len();
    let n = grid.len();
    let max_x = m - 1;
    let max_y = n - 1;

    let mut visited = vec![0u8; m * n];
    let mut position = start_position.clone();
    let mut visited_count = 0;

    loop {
        let (x, y, direction) = position;
        let i = y * m + x;

        if visited[i] & direction > 0 {
            return SimulationResult::Cycle;
        }
        if visited[i] == 0 {
            visited_count += 1;
        }
        visited[i] |= direction;

        let (next_x, next_y) = match position {
            (x, y, UP) if y > 0 => (x, y - 1),
            (x, y, RIGHT) if x < max_x => (x + 1, y),
            (x, y, DOWN) if y < max_y => (x, y + 1),
            (x, y, LEFT) if x > 0 => (x - 1, y),
            _ => return SimulationResult::Escaped(visited_count),
        };

        position = match (&grid[next_y][next_x], direction) {
            (Cell::Empty, _) => (next_x, next_y, direction),
            (Cell::Obstacle, UP) => (x, y, RIGHT),
            (Cell::Obstacle, RIGHT) => (x, y, DOWN),
            (Cell::Obstacle, DOWN) => (x, y, LEFT),
            (Cell::Obstacle, LEFT) => (x, y, UP),
            _ => panic!(""),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 6);
    }
}
