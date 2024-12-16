use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

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

#[derive(Eq, PartialEq)]
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
            if input[next_index] == b'#' {
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
