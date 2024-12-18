use dary_heap::BinaryHeap;
use std::cmp::Ordering;
use std::fmt::Display;

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const GRID_SIZE: usize = WIDTH * HEIGHT;
const START_X: usize = 0;
const START_Y: usize = 0;
const START_INDEX: usize = START_Y * WIDTH + START_X;
const END_X: usize = 70;
const END_Y: usize = 70;
const END_INDEX: usize = END_Y * WIDTH + END_X;
const PART1_BYTES: usize = 1024;

macro_rules! read_0_99_and_skip_next {
    ($input:expr, $i:expr) => {{
        match $input.get_unchecked($i + 1) {
            b'0'..=b'9' => {
                $i += 3;
                (*$input.get_unchecked($i - 3) as usize) * 10
                    + (*$input.get_unchecked($i - 2) as usize)
                    - 528
            }
            _ => {
                $i += 2;
                (*$input.get_unchecked($i - 2) as usize) - 48
            }
        }
    }};
}

macro_rules! index {
    ($x:expr, $y:expr) => {
        $y * WIDTH + $x
    };
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    index: usize,
    score: usize,
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

unsafe fn get_directions(index: usize) -> Vec<usize> {
    let mut directions = Vec::with_capacity(4);

    if index >= WIDTH {
        directions.push(index - WIDTH);
    }

    if index % WIDTH != 0 {
        directions.push(index - 1);
    }

    if index % WIDTH != WIDTH - 1 {
        directions.push(index + 1);
    }

    if index < GRID_SIZE - WIDTH {
        directions.push(index + WIDTH);
    }

    directions
}

unsafe fn find_fastest_path_score(grid: &[u8; GRID_SIZE]) -> usize {
    let mut open_set = BinaryHeap::with_capacity(1000);
    let mut closed_set = [false; GRID_SIZE];
    let mut best_scores = [usize::MAX; GRID_SIZE];

    let start_node = Node { score: 0, index: 0 };
    open_set.push(start_node);
    best_scores[START_INDEX] = 0;

    while let Some(current) = open_set.pop() {
        if current.index == END_INDEX {
            return current.score;
        }

        if current.score > best_scores[current.index] {
            continue;
        }

        for next_index in get_directions(current.index) {
            if *grid.get_unchecked(next_index) == 1 || closed_set[next_index] {
                continue;
            }

            let next_score = current.score + 1;

            if next_score < best_scores[next_index] {
                best_scores[next_index] = next_score;
                open_set.push(Node {
                    score: next_score,
                    index: next_index,
                });
            }
        }

        closed_set[current.index] = true;
    }

    usize::MAX
}

pub fn part1(input: &str) -> impl Display {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut grid = [0u8; GRID_SIZE];
    let mut i = 0;

    macro_rules! corrupt {
        () => {
            grid[index!(
                read_0_99_and_skip_next!(input, i),
                read_0_99_and_skip_next!(input, i)
            )] = 1;
        };
    }

    let mut c = 0;
    while c < PART1_BYTES {
        corrupt!();
        corrupt!();
        corrupt!();
        corrupt!();
        corrupt!();
        corrupt!();
        corrupt!();
        corrupt!();
        c += 8;
    }

    find_fastest_path_score(&grid)
}

pub fn part2(input: &str) -> impl Display {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut initial_grid = [0u8; GRID_SIZE];
    let mut i = 0;

    let mut corrupted = Vec::with_capacity(3500);
    while i < input.len() {
        corrupted.push(index!(
            read_0_99_and_skip_next!(input, i),
            read_0_99_and_skip_next!(input, i)
        ));
    }

    let mut c = 0;
    while c < PART1_BYTES {
        initial_grid[corrupted[c]] = 1;
        c += 1;
    }

    let mut left = PART1_BYTES;
    let mut right = corrupted.len();
    let mut grid = initial_grid.clone();

    while left < right {
        let mid = (left + right) / 2;

        for i in PART1_BYTES..mid {
            grid[corrupted[i]] = 1;
        }

        if find_fastest_path_score(&grid) == usize::MAX {
            right = mid;
        } else {
            left = mid + 1;
        }

        grid.copy_from_slice(&initial_grid);
    }

    format!(
        "{},{}",
        corrupted[left - 1] / WIDTH,
        corrupted[left - 1] % WIDTH
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day18_part1() {
        let prod_input = read_to_string("./inputs/18.txt").unwrap();
        let prod_output = read_to_string("./outputs/18p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day18_part2() {
        let prod_input = read_to_string("./inputs/18.txt").unwrap();
        let prod_output = read_to_string("./outputs/18p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
