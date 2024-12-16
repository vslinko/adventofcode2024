use dary_heap::BinaryHeap;
use std::cmp::Ordering;
use std::collections::VecDeque;

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

struct NextDirection {
    delta: isize,
    direction: Direction,
    score: usize,
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

const NEXT_DIRECTIONS1: [[NextDirection; 3]; 4] = [
    // north
    [
        NextDirection {
            delta: LINE_LENGTH as isize * -1,
            direction: Direction::North,
            score: 1,
        },
        NextDirection {
            delta: -1,
            direction: Direction::West,
            score: 1001,
        },
        NextDirection {
            delta: 1,
            direction: Direction::East,
            score: 1001,
        },
    ],
    // south
    [
        NextDirection {
            delta: LINE_LENGTH as isize,
            direction: Direction::South,
            score: 1,
        },
        NextDirection {
            delta: -1,
            direction: Direction::West,
            score: 1001,
        },
        NextDirection {
            delta: 1,
            direction: Direction::East,
            score: 1001,
        },
    ],
    // east
    [
        NextDirection {
            delta: 1,
            direction: Direction::East,
            score: 1,
        },
        NextDirection {
            delta: LINE_LENGTH as isize * -1,
            direction: Direction::North,
            score: 1001,
        },
        NextDirection {
            delta: LINE_LENGTH as isize,
            direction: Direction::South,
            score: 1001,
        },
    ],
    // west
    [
        NextDirection {
            delta: -1,
            direction: Direction::West,
            score: 1,
        },
        NextDirection {
            delta: LINE_LENGTH as isize * -1,
            direction: Direction::North,
            score: 1001,
        },
        NextDirection {
            delta: LINE_LENGTH as isize,
            direction: Direction::South,
            score: 1001,
        },
    ],
];

unsafe fn find_fastest_path_score(input: &[u8]) -> usize {
    let mut open_set = BinaryHeap::with_capacity(1000);
    let mut closed_set = [false; LINE_LENGTH * HEIGHT];
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

        for dir in NEXT_DIRECTIONS1[current.direction as usize].iter() {
            let next_index = (current.index as isize + dir.delta) as usize;

            if *input.get_unchecked(next_index) == b'#' || closed_set[next_index] {
                continue;
            }

            let new_score = current.score + dir.score;

            if new_score < best_scores[next_index] {
                best_scores[next_index] = new_score;

                let next_node = Node {
                    score: new_score,
                    index: next_index,
                    direction: dir.direction,
                };

                open_set.push(next_node);
            }
        }

        closed_set[current.index] = true
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

const NEXT_DIRECTIONS2: [[NextDirection; 3]; 4] = [
    // north
    [
        NextDirection {
            delta: LINE_LENGTH as isize * -1,
            direction: Direction::North,
            score: 1,
        },
        NextDirection {
            delta: 0,
            direction: Direction::West,
            score: 1000,
        },
        NextDirection {
            delta: 0,
            direction: Direction::East,
            score: 1000,
        },
    ],
    // south
    [
        NextDirection {
            delta: LINE_LENGTH as isize,
            direction: Direction::South,
            score: 1,
        },
        NextDirection {
            delta: 0,
            direction: Direction::West,
            score: 1000,
        },
        NextDirection {
            delta: 0,
            direction: Direction::East,
            score: 1000,
        },
    ],
    // east
    [
        NextDirection {
            delta: 1,
            direction: Direction::East,
            score: 1,
        },
        NextDirection {
            delta: 0,
            direction: Direction::North,
            score: 1000,
        },
        NextDirection {
            delta: 0,
            direction: Direction::South,
            score: 1000,
        },
    ],
    // west
    [
        NextDirection {
            delta: -1,
            direction: Direction::West,
            score: 1,
        },
        NextDirection {
            delta: 0,
            direction: Direction::North,
            score: 1000,
        },
        NextDirection {
            delta: 0,
            direction: Direction::South,
            score: 1000,
        },
    ],
];

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

        for dir in NEXT_DIRECTIONS2[item.direction as usize].iter() {
            let next_index = (current_index as isize + dir.delta) as usize;
            let next_score = item.score + dir.score;

            if *input.get_unchecked(next_index) == b'#' || next_score > max_score {
                continue;
            }

            let scores_index = get_scores_index(next_index, dir.direction);
            let current_score = scores[scores_index];

            if next_score <= current_score {
                scores[scores_index] = next_score;

                let mut next_path = item.path.clone();
                next_path.push(next_index);
                queue.push_back(QueueItem {
                    path: next_path,
                    direction: dir.direction,
                    score: next_score,
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
