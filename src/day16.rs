use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

const WIDTH: usize = 141;
const HEIGHT: usize = 141;
const LINE_LENGTH: usize = WIDTH + 1;
const START_X: usize = 1;
const START_Y: usize = HEIGHT - 2;
const END_X: usize = WIDTH - 2;
const END_Y: usize = 1;

#[derive(Eq, PartialEq)]
struct Node {
    f_score: usize,
    g_score: usize,
    x: usize,
    y: usize,
    index: usize,
    direction: Direction,
}

#[derive(Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| other.g_score.cmp(&self.g_score))
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_index(x: usize, y: usize) -> usize {
    y * LINE_LENGTH + x
}

fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
}

fn get_neighbors_with_direction(
    x: usize,
    y: usize,
    current_dir: Direction,
) -> [(usize, usize, Direction, usize); 3] {
    match current_dir {
        Direction::East => [
            (x + 1, y, Direction::East, 1),
            (x, y - 1, Direction::North, 1001),
            (x, y + 1, Direction::South, 1001),
        ],
        Direction::West => [
            (x - 1, y, Direction::West, 1),
            (x, y - 1, Direction::North, 1001),
            (x, y + 1, Direction::South, 1001),
        ],
        Direction::North => [
            (x, y - 1, Direction::North, 1),
            (x - 1, y, Direction::West, 1001),
            (x + 1, y, Direction::East, 1001),
        ],
        Direction::South => [
            (x, y + 1, Direction::South, 1),
            (x - 1, y, Direction::West, 1001),
            (x + 1, y, Direction::East, 1001),
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
        f_score: manhattan_distance(START_X, START_Y, END_X, END_Y),
        g_score: 0,
        x: START_X,
        y: START_Y,
        index: get_index(START_X, START_Y),
        direction: Direction::East,
    };
    open_set.push(start_node);

    while let Some(current) = open_set.pop() {
        if current.x == END_X && current.y == END_Y {
            return current.g_score;
        }

        if !closed_set.insert(current.index) {
            continue;
        }

        for (next_x, next_y, next_dir, next_score) in
            get_neighbors_with_direction(current.x, current.y, current.direction)
        {
            let next_index = get_index(next_x, next_y);

            if input[next_index] == b'#' {
                continue;
            }

            if closed_set.contains(&next_index) {
                continue;
            }

            let next_g_score = current.g_score + next_score;
            let next_h_score = manhattan_distance(next_x, next_y, END_X, END_Y);

            let next_node = Node {
                f_score: next_g_score + next_h_score,
                g_score: next_g_score,
                x: next_x,
                y: next_y,
                index: next_index,
                direction: next_dir,
            };

            open_set.push(next_node);
        }
    }

    usize::MAX
}

pub fn part2(input: &str) -> i32 {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> i32 {
    123
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
