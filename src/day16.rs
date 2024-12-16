use dary_heap::BinaryHeap;
use std::cmp::Ordering;
use std::collections::VecDeque;

use rustc_hash::{FxBuildHasher, FxHashSet};

const WIDTH: usize = 141;
const HEIGHT: usize = 141;
const LINE_LENGTH: usize = WIDTH + 1;
const START_X: usize = 1;
const START_Y: usize = HEIGHT - 2;
const START_INDEX: usize = START_Y * LINE_LENGTH + START_X;
const END_X: usize = WIDTH - 2;
const END_Y: usize = 1;
const END_INDEX: usize = END_Y * LINE_LENGTH + END_X;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Eq, PartialEq)]
struct Node {
    score: usize,
    index: usize,
    direction: Direction,
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

fn get_neighbors_with_direction1(
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

unsafe fn find_fastest_path_score(input: &[u8]) -> usize {
    let mut open_set = BinaryHeap::with_capacity(1000);
    let mut closed_set = FxHashSet::with_capacity_and_hasher(1000, FxBuildHasher::default());
    let mut best_scores = [usize::MAX; LINE_LENGTH * HEIGHT];

    let start_node = Node {
        score: 0,
        index: START_INDEX,
        direction: Direction::East,
    };
    open_set.push(start_node);
    best_scores[START_INDEX] = 0;

    while let Some(current) = open_set.pop() {
        if current.index == END_INDEX {
            return current.score;
        }

        if current.score > best_scores[current.index] {
            continue;
        }

        for (next_index, next_dir, next_score) in
            get_neighbors_with_direction1(current.index, current.direction)
        {
            if *input.get_unchecked(next_index) == b'#' || closed_set.contains(&next_index) {
                continue;
            }

            let new_score = current.score + next_score;

            if new_score < best_scores[next_index] {
                best_scores[next_index] = new_score;

                let next_node = Node {
                    score: new_score,
                    index: next_index,
                    direction: next_dir,
                };

                open_set.push(next_node);
            }
        }

        closed_set.insert(current.index);
    }

    usize::MAX
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let input = input.as_bytes();

    find_fastest_path_score(input)
}

fn get_neighbors_with_direction2(
    index: usize,
    direction: Direction,
) -> [(usize, Direction, usize); 3] {
    match direction {
        Direction::East => [
            (index + 1, Direction::East, 1),
            (index, Direction::North, 1000),
            (index, Direction::South, 1000),
        ],
        Direction::West => [
            (index - 1, Direction::West, 1),
            (index, Direction::North, 1000),
            (index, Direction::South, 1000),
        ],
        Direction::North => [
            (index - LINE_LENGTH, Direction::North, 1),
            (index, Direction::West, 1000),
            (index, Direction::East, 1000),
        ],
        Direction::South => [
            (index + LINE_LENGTH, Direction::South, 1),
            (index, Direction::West, 1000),
            (index, Direction::East, 1000),
        ],
    }
}

fn get_scores_index(index: usize, direction: Direction) -> usize {
    const ROW_SIZE1: usize = LINE_LENGTH * HEIGHT;
    const ROW_SIZE2: usize = LINE_LENGTH * HEIGHT * 2;
    const ROW_SIZE3: usize = LINE_LENGTH * HEIGHT * 3;

    match direction {
        Direction::East => index,
        Direction::West => index + ROW_SIZE1,
        Direction::North => index + ROW_SIZE2,
        Direction::South => index + ROW_SIZE3,
    }
}

struct QueueItem {
    path: smallvec::SmallVec<[usize; 8]>,
    direction: Direction,
    score: usize,
}

unsafe fn find_unique_cells_count_of_all_fastest_pathes(input: &[u8], max_score: usize) -> usize {
    let mut unique_cells = Vec::with_capacity((LINE_LENGTH * HEIGHT) / 2);
    let mut queue = VecDeque::with_capacity((LINE_LENGTH * HEIGHT) / 4);
    let mut scores = [usize::MAX; LINE_LENGTH * HEIGHT * 4];

    queue.push_back(QueueItem {
        path: smallvec::SmallVec::<[usize; 8]>::from_slice(&[START_INDEX]),
        direction: Direction::East,
        score: 0,
    });
    scores[get_scores_index(START_INDEX, Direction::East)] = 0;

    while let Some(item) = queue.pop_front() {
        let current_index = *item.path.last().unwrap_unchecked();

        if current_index == END_INDEX {
            unique_cells.extend(item.path.iter().copied());
            continue;
        }

        for (next_index, next_direction, next_score) in
            get_neighbors_with_direction2(current_index, item.direction)
        {
            if *input.get_unchecked(next_index) == b'#' || item.score + next_score > max_score {
                continue;
            }

            let new_score = item.score + next_score;
            let scores_index = get_scores_index(next_index, next_direction);
            let current_score = scores[scores_index];

            if new_score <= current_score {
                scores[scores_index] = new_score;

                let mut new_path = item.path.clone();
                new_path.push(next_index);
                queue.push_back(QueueItem {
                    path: new_path,
                    direction: next_direction,
                    score: new_score,
                });
            }
        }
    }

    unique_cells.sort_unstable();
    unique_cells.dedup();
    unique_cells.len()
}

pub fn part2(input: &str) -> usize {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> usize {
    let input = input.as_bytes();

    let max_score = find_fastest_path_score(&input);

    find_unique_cells_count_of_all_fastest_pathes(&input, max_score)
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
