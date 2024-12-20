use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const SAVED_TIME_LIMIT: i32 = 100;

#[derive(Eq, PartialEq)]
struct Node {
    score: i32,
    position: (i32, i32),
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

fn find_fastest_path(
    grid: &Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
) -> Option<(i32, Vec<(i32, i32)>)> {
    let mut queue = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut scores: HashMap<(i32, i32), i32> = HashMap::new();

    queue.push(Node {
        score: 0,
        position: start,
    });
    scores.insert(start, 0);

    while let Some(Node {
        score: _,
        position: current,
    }) = queue.pop()
    {
        if current == end {
            let score = *scores.get(&current).unwrap();
            let mut path = vec![current];
            let mut current_pos = current;

            while let Some(&prev) = came_from.get(&current_pos) {
                path.insert(0, prev);
                current_pos = prev;
            }

            return Some((score, path));
        }

        let node_score = scores[&current];
        let (x, y) = current;

        let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

        for &neighbor in neighbors.iter() {
            let (nx, ny) = neighbor;

            if nx < 0
                || ny < 0
                || nx >= grid[0].len() as i32
                || ny >= grid.len() as i32
                || grid[ny as usize][nx as usize] == '#'
            {
                continue;
            }

            let new_score = node_score + 1;
            let neighbor_score = scores.get(&neighbor).copied().unwrap_or(i32::MAX);

            if new_score < neighbor_score {
                came_from.insert(neighbor, current);
                scores.insert(neighbor, new_score);

                let heuristic = (nx - end.0).abs() + (ny - end.1).abs();
                queue.push(Node {
                    score: -(new_score + heuristic),
                    position: neighbor,
                });
            }
        }
    }

    None
}

fn solve(input: &str, cheat_size: i32) -> i32 {
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let m = grid[0].len() as i32;
    let n = grid.len() as i32;

    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..n {
        for x in 0..m {
            match grid[y as usize][x as usize] {
                'S' => start = (x, y),
                'E' => end = (x, y),
                _ => continue,
            }
        }
    }

    let (initial_score, initial_path) =
        find_fastest_path(&grid, start, end).expect("No initial path found");

    let mut result = 0;
    let mut cache: HashMap<(i32, i32), i32> = HashMap::new();

    for (time_spent, &pos) in initial_path.iter().enumerate() {
        cache.insert(pos, initial_score - time_spent as i32);
    }

    for (time_spent, &(x, y)) in initial_path.iter().enumerate() {
        let x_from = 0.max(x - 20);
        let x_to = (m - 1).min(x + 20);
        let y_from = 0.max(y - 20);
        let y_to = (n - 1).min(y + 20);

        for ny in y_from..=y_to {
            for nx in x_from..=x_to {
                if (nx, ny) == (x, y) || grid[ny as usize][nx as usize] == '#' {
                    continue;
                }

                let dist = (nx - x).abs() + (ny - y).abs();

                if dist > cheat_size {
                    continue;
                }

                let score_after_cheating = match cache.get(&(nx, ny)) {
                    Some(&score) => score,
                    None => {
                        if let Some((score, _)) = find_fastest_path(&grid, (nx, ny), end) {
                            cache.insert((nx, ny), score);
                            score
                        } else {
                            continue;
                        }
                    }
                };

                let total_time = time_spent as i32 + dist + score_after_cheating;
                let saved_time = initial_score - total_time;

                if saved_time >= SAVED_TIME_LIMIT {
                    result += 1;
                }
            }
        }
    }

    result
}

pub fn part1(input: &str) -> i32 {
    solve(input, 2)
}

pub fn part2(input: &str) -> i32 {
    solve(input, 20)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

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
