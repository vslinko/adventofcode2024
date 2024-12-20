use dary_heap::BinaryHeap;
use std::cmp::Ordering;

const WIDTH: usize = 141;
const HEIGHT: usize = 141;
const LINE_LENGTH: usize = WIDTH + 1;
const GRID_SIZE: usize = LINE_LENGTH * HEIGHT;
const SAVED_TIME_LIMIT: usize = 100;
const NO_PATH_FOUND: usize = usize::MAX;
const EMPTY_CACHE: usize = usize::MAX - 1;

#[derive(Eq, PartialEq)]
struct Node {
    score: usize,
    pos: usize,
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
    let mut result = [0; GRID_SIZE * 4];
    let mut i = 0;

    while i < GRID_SIZE {
        let pos_x = i % LINE_LENGTH;
        let pos_y = i / LINE_LENGTH;

        if pos_x > 0 {
            result[i * 4] = i - 1;
        }
        if pos_x < WIDTH {
            result[i * 4 + 1] = i + 1;
        }
        if pos_y > 0 {
            result[i * 4 + 2] = i - LINE_LENGTH;
        }
        if pos_y < HEIGHT {
            result[i * 4 + 3] = i + LINE_LENGTH;
        }

        i += 1;
    }

    result
};

fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize
}

fn find_fastest_path(grid: &[u8], start: usize, end: usize) -> (usize, Vec<usize>) {
    let mut queue = BinaryHeap::new();
    let mut came_from = [EMPTY_CACHE; GRID_SIZE];
    let mut scores = [usize::MAX; GRID_SIZE];
    let mut closed_set = [false; GRID_SIZE];

    queue.push(Node {
        score: 0,
        pos: start,
    });
    scores[start] = 0;

    while let Some(current) = queue.pop() {
        if current.pos == end {
            let mut path = vec![current.pos];
            let mut pos = current.pos;

            while came_from[pos] != EMPTY_CACHE {
                pos = came_from[pos];
                path.insert(0, pos);
            }

            return (current.score, path);
        }

        for i in 0..4 {
            let neighbor = DIRECTIONS[current.pos * 4 + i];

            if closed_set[neighbor] || grid[neighbor] == b'#' {
                continue;
            }

            let new_score = scores[current.pos] + 1;

            if new_score < scores[neighbor] {
                came_from[neighbor] = current.pos;
                scores[neighbor] = new_score;

                queue.push(Node {
                    score: new_score,
                    pos: neighbor,
                });
            }
        }

        closed_set[current.pos] = true;
    }

    (NO_PATH_FOUND, Vec::new())
}

unsafe fn solve(input: &str, cheat_size: usize) -> usize {
    let grid = input.as_bytes();

    let start = grid.iter().position(|&c| c == b'S').unwrap_unchecked();
    let end = grid.iter().position(|&c| c == b'E').unwrap_unchecked();

    let (initial_total_time, initial_path) = find_fastest_path(&grid, start, end);

    let mut result = 0;
    let mut cache = [EMPTY_CACHE; GRID_SIZE];

    for (time_before_cheating, &pos) in initial_path.iter().enumerate() {
        cache[pos] = initial_total_time - time_before_cheating;
    }

    for (time_before_cheating, &pos) in initial_path.iter().enumerate() {
        let pos_x = pos % LINE_LENGTH;
        let pos_y = pos / LINE_LENGTH;

        let x_from = if pos_x > 21 { pos_x - 20 } else { 1 };
        let x_to = (WIDTH - 2).min(pos_x + 20);
        let y_from = if pos_y > 21 { pos_y - 20 } else { 1 };
        let y_to = (HEIGHT - 2).min(pos_y + 20);

        for ny in y_from..=y_to {
            for nx in x_from..=x_to {
                let next_pos = ny * LINE_LENGTH + nx;

                if next_pos == pos || grid[next_pos] == b'#' {
                    continue;
                }

                let cheating_time = distance(nx, ny, pos_x, pos_y);

                if cheating_time > cheat_size {
                    continue;
                }

                let time_after_cheating = match cache[next_pos] {
                    EMPTY_CACHE => {
                        let (score, _) = find_fastest_path(&grid, next_pos, end);
                        cache[next_pos] = score;
                        score
                    }
                    score => score,
                };

                if time_after_cheating == usize::MAX {
                    continue;
                }

                let total_time = time_before_cheating + cheating_time + time_after_cheating;

                if total_time > initial_total_time {
                    continue;
                }

                let saved_time = initial_total_time - total_time;

                if saved_time < SAVED_TIME_LIMIT {
                    continue;
                }

                result += 1;
            }
        }
    }

    result
}

pub fn part1(input: &str) -> usize {
    unsafe { solve(input, 2) }
}

pub fn part2(input: &str) -> usize {
    unsafe { solve(input, 20) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day20_part1_binary_heap_ordering() {
        let mut heap = BinaryHeap::new();
        heap.push(Node { score: 1, pos: 0 });
        heap.push(Node { score: 3, pos: 0 });
        heap.push(Node { score: 2, pos: 0 });

        assert_eq!(heap.pop().unwrap().score, 1);
        assert_eq!(heap.pop().unwrap().score, 2);
        assert_eq!(heap.pop().unwrap().score, 3);
    }

    #[test]
    fn test_day20_part1() {
        let prod_input = read_to_string("./inputs/20.txt").unwrap();
        let prod_output = read_to_string("./outputs/20p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day20_part2() {
        let prod_input = read_to_string("./inputs/20.txt").unwrap();
        let prod_output = read_to_string("./outputs/20p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
