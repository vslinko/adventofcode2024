use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::collections::{HashMap, VecDeque};

const WIDTH: usize = 141;
const HEIGHT: usize = 141;
const LINE_LENGTH: usize = WIDTH + 1;
const START_X: usize = 1;
const START_Y: usize = HEIGHT - 2;
const START_INDEX: usize = START_Y * LINE_LENGTH + START_X;
const END_X: usize = WIDTH - 2;
const END_Y: usize = 1;
const END_INDEX: usize = END_Y * LINE_LENGTH + END_X;

#[derive(Eq, PartialEq)]
struct Node {
    score: usize,
    index: usize,
    direction: Direction,
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors_with_direction(
    index: usize,
    current_dir: Direction,
) -> [(usize, Direction, usize); 3] {
    match current_dir {
        Direction::East => [
            (index + 1, Direction::East, 1),
            (index - LINE_LENGTH, Direction::North, 1001),
            (index + LINE_LENGTH, Direction::South, 1001),
        ],
        Direction::West => [
            (index - 1, Direction::West, 1),
            (index - LINE_LENGTH, Direction::North, 1001),
            (index + LINE_LENGTH, Direction::South, 1001),
        ],
        Direction::North => [
            (index - LINE_LENGTH, Direction::North, 1),
            (index - 1, Direction::West, 1001),
            (index + 1, Direction::East, 1001),
        ],
        Direction::South => [
            (index + LINE_LENGTH, Direction::South, 1),
            (index - 1, Direction::West, 1001),
            (index + 1, Direction::East, 1001),
        ],
    }
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let input = input.as_bytes();

    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();

    let start_node = Node {
        score: 0,
        index: START_INDEX,
        direction: Direction::East,
    };
    open_set.push(start_node);

    while let Some(current) = open_set.pop() {
        if current.index == END_INDEX {
            return current.score;
        }

        if !closed_set.insert(current.index) {
            continue;
        }

        for (next_index, next_dir, next_score) in
            get_neighbors_with_direction(current.index, current.direction)
        {
            if *input.get_unchecked(next_index) == b'#' {
                continue;
            }

            if closed_set.contains(&next_index) {
                continue;
            }

            let next_node = Node {
                score: current.score + next_score,
                index: next_index,
                direction: next_dir,
            };

            open_set.push(next_node);
        }
    }

    usize::MAX
}

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Point {
    fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    fn hash(&self) -> String {
        let direction = match self.direction {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::East => 'E',
            Direction::West => 'W',
        };

        format!("{},{},{}", self.x, self.y, direction)
    }
}

struct DirectionMove {
    direction: Direction,
    dx: i32,
    dy: i32,
    score: i32,
}

fn get_directions() -> HashMap<Direction, Vec<DirectionMove>> {
    let mut directions = HashMap::new();

    directions.insert(
        Direction::East,
        vec![
            DirectionMove {
                direction: Direction::East,
                dx: 1,
                dy: 0,
                score: 1,
            },
            DirectionMove {
                direction: Direction::North,
                dx: 0,
                dy: 0,
                score: 1000,
            },
            DirectionMove {
                direction: Direction::South,
                dx: 0,
                dy: 0,
                score: 1000,
            },
        ],
    );

    directions.insert(
        Direction::North,
        vec![
            DirectionMove {
                direction: Direction::North,
                dx: 0,
                dy: -1,
                score: 1,
            },
            DirectionMove {
                direction: Direction::West,
                dx: 0,
                dy: 0,
                score: 1000,
            },
            DirectionMove {
                direction: Direction::East,
                dx: 0,
                dy: 0,
                score: 1000,
            },
        ],
    );

    directions.insert(
        Direction::West,
        vec![
            DirectionMove {
                direction: Direction::West,
                dx: -1,
                dy: 0,
                score: 1,
            },
            DirectionMove {
                direction: Direction::North,
                dx: 0,
                dy: 0,
                score: 1000,
            },
            DirectionMove {
                direction: Direction::South,
                dx: 0,
                dy: 0,
                score: 1000,
            },
        ],
    );

    directions.insert(
        Direction::South,
        vec![
            DirectionMove {
                direction: Direction::South,
                dx: 0,
                dy: 1,
                score: 1,
            },
            DirectionMove {
                direction: Direction::West,
                dx: 0,
                dy: 0,
                score: 1000,
            },
            DirectionMove {
                direction: Direction::East,
                dx: 0,
                dy: 0,
                score: 1000,
            },
        ],
    );

    directions
}

fn find_all_shortest_paths(
    maze: &Vec<Vec<char>>,
    start_x: i32,
    start_y: i32,
    start_direction: Direction,
    end_x: i32,
    end_y: i32,
) -> Vec<(Vec<Point>, i32)> {
    let start = Point::new(start_x, start_y, start_direction);
    let directions = get_directions();
    let mut all_paths = Vec::new();
    let mut queue = VecDeque::new();
    let mut scores = HashMap::new();

    queue.push_back((vec![start.clone()], 0));
    scores.insert(start.hash(), 0);

    while let Some((current_path, score)) = queue.pop_front() {
        let current = current_path.last().unwrap();

        if current.x == end_x && current.y == end_y {
            all_paths.push((current_path, score));
            continue;
        }

        for d in directions.get(&current.direction).unwrap() {
            let next_x = current.x + d.dx;
            let next_y = current.y + d.dy;

            if maze[next_y as usize][next_x as usize] != '#' {
                let next_point = Point::new(next_x, next_y, d.direction.clone());
                let new_distance = score + d.score;

                let current_score = scores.get(&next_point.hash()).unwrap_or(&i32::MAX);
                if new_distance <= *current_score {
                    scores.insert(next_point.hash(), new_distance);
                    let mut new_path = current_path.clone();
                    new_path.push(next_point);
                    queue.push_back((new_path, new_distance));
                }
            }
        }
    }

    let min_score = all_paths.iter().map(|(_, score)| *score).min().unwrap_or(0);

    all_paths
        .into_iter()
        .filter(|(_, score)| *score == min_score)
        .collect()
}

pub fn part2(input: &str) -> usize {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    assert!(!lines.is_empty());

    let m = lines[0].len();
    let n = lines.len();

    let start_x = 1;
    let start_y = (n - 2) as i32;
    let end_x = (m - 2) as i32;
    let end_y = 1;

    assert_eq!(lines[start_y as usize][start_x], 'S');
    assert_eq!(lines[end_y as usize][end_x as usize], 'E');

    let mut lines = lines;
    lines[start_y as usize][start_x] = '.';
    lines[end_y as usize][end_x as usize] = '.';

    let shortest_paths = find_all_shortest_paths(
        &lines,
        start_x as i32,
        start_y,
        Direction::East,
        end_x,
        end_y,
    );

    let mut unique_cells = HashSet::new();

    for (path, _) in shortest_paths {
        for point in path {
            unique_cells.insert(format!("{},{}", point.x, point.y));
        }
    }

    unique_cells.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day16_part1() {
        let prod_input = read_to_string("./inputs/16.txt").unwrap();
        let prod_output = read_to_string("./outputs/16p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day16_part2() {
        let prod_input = read_to_string("./inputs/16.txt").unwrap();
        let prod_output = read_to_string("./outputs/16p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
