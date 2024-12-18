use dary_heap::BinaryHeap;
use std::cmp::Ordering;

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

const DIRECTIONS: [usize; GRID_SIZE * 4] = {
    let mut dirs = [0; GRID_SIZE * 4];
    let mut i = 0;

    while i < GRID_SIZE {
        if i >= WIDTH {
            dirs[i * 4] = i - WIDTH;
        }

        if i % WIDTH != 0 {
            dirs[i * 4 + 1] = i - 1;
        }

        if i % WIDTH != WIDTH - 1 {
            dirs[i * 4 + 2] = i + 1;
        }

        if i < GRID_SIZE - WIDTH {
            dirs[i * 4 + 3] = i + WIDTH;
        }

        i += 1;
    }

    dirs
};

unsafe fn find_fastest_path_score(grid: &[bool; GRID_SIZE]) -> usize {
    let mut open_set = BinaryHeap::with_capacity(1000);
    let mut closed_set = [true; GRID_SIZE];
    let mut best_scores = [usize::MAX; GRID_SIZE];

    open_set.push(Node { score: 0, index: 0 });
    *best_scores.get_unchecked_mut(START_INDEX) = 0;
    *closed_set.get_unchecked_mut(START_INDEX) = false;

    while let Some(current) = open_set.pop() {
        if current.index == END_INDEX {
            return current.score;
        }

        if current.score > *best_scores.get_unchecked(current.index) {
            continue;
        }

        macro_rules! check_direction {
            ($next_index:expr) => {
                let next_index = $next_index;

                if *closed_set.get_unchecked(next_index) && *grid.get_unchecked(next_index) {
                    let next_score = current.score + 1;

                    if next_score < *best_scores.get_unchecked(next_index) {
                        *best_scores.get_unchecked_mut(next_index) = next_score;

                        open_set.push(Node {
                            score: next_score,
                            index: next_index,
                        });
                    }
                }
            };
        }

        check_direction!(*DIRECTIONS.get_unchecked(current.index * 4));
        check_direction!(*DIRECTIONS.get_unchecked(current.index * 4 + 1));
        check_direction!(*DIRECTIONS.get_unchecked(current.index * 4 + 2));
        check_direction!(*DIRECTIONS.get_unchecked(current.index * 4 + 3));

        *closed_set.get_unchecked_mut(current.index) = false;
    }

    usize::MAX
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut grid = [true; GRID_SIZE];
    let mut i = 0;

    macro_rules! corrupt {
        () => {
            *grid.get_unchecked_mut(index!(
                read_0_99_and_skip_next!(input, i),
                read_0_99_and_skip_next!(input, i)
            )) = false;
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

pub fn part2(input: &str) -> String {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> String {
    let input = input.as_bytes();
    let mut initial_grid = [true; GRID_SIZE];
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
        *initial_grid.get_unchecked_mut(*corrupted.get_unchecked(c)) = false;
        c += 1;
    }

    let mut left = PART1_BYTES;
    let mut right = corrupted.len();
    let mut grid = initial_grid.clone();

    while left < right {
        let mid = (left + right) / 2;

        for i in PART1_BYTES..mid {
            *grid.get_unchecked_mut(*corrupted.get_unchecked(i)) = false;
        }

        if find_fastest_path_score(&grid) == usize::MAX {
            right = mid;
        } else {
            left = mid + 1;
        }

        grid.copy_from_slice(&initial_grid);
    }

    (corrupted.get_unchecked(left - 1) / WIDTH).to_string()
        + ","
        + &(corrupted.get_unchecked(left - 1) % WIDTH).to_string()
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
