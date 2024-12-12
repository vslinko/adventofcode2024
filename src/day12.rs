use rayon::prelude::*;
use rustc_hash::{FxBuildHasher, FxHashSet};

struct Region {
    plots: FxHashSet<i32>,
    width: i32,
    height: i32,
}

fn r_index(x: i32, y: i32, width: i32) -> i32 {
    y * width + x
}

fn r_x(index: i32, width: i32) -> i32 {
    index % width
}

fn r_y(index: i32, width: i32) -> i32 {
    index / width
}

impl Region {
    fn new(width: i32, height: i32) -> Self {
        Region {
            plots: FxHashSet::with_capacity_and_hasher(100, FxBuildHasher::default()),
            width,
            height,
        }
    }

    fn add_plot(&mut self, x: i32, y: i32) {
        self.plots.insert(r_index(x, y, self.width));
    }

    fn has_plot(&self, x: i32, y: i32) -> bool {
        self.plots.contains(&r_index(x, y, self.width))
    }

    fn area(&self) -> u64 {
        self.plots.len() as u64
    }

    fn perimeter(&self) -> u64 {
        let mut perimeter = 0;

        for &i in self.plots.iter() {
            let x = r_x(i, self.width);
            let y = r_y(i, self.width);

            if x == 0 || !self.plots.contains(&(i - 1)) {
                perimeter += 1;
            }
            if x == self.width - 1 || !self.plots.contains(&(i + 1)) {
                perimeter += 1;
            }
            if y == 0 || !self.plots.contains(&(i - self.width)) {
                perimeter += 1;
            }
            if y == self.height - 1 || !self.plots.contains(&(i + self.width)) {
                perimeter += 1;
            }
        }

        perimeter
    }

    fn sides(&self) -> u64 {
        let mut sides = 0;

        let mut plots = self
            .plots
            .iter()
            .map(|&i| (r_x(i, self.width), r_y(i, self.width)))
            .collect::<Vec<_>>();

        plots.sort_unstable_by(|a, b| a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0)));

        let mut prev_y = 0;
        let mut prev_north = -2;
        let mut prev_south = -2;

        for &(x, y) in plots.iter() {
            if y != prev_y {
                prev_north = -2;
                prev_south = -2;
                prev_y = y;
            }

            if !self.has_plot(x, y - 1) {
                if x - prev_north > 1 {
                    sides += 1;
                }
                prev_north = x;
            }

            if !self.has_plot(x, y + 1) {
                if x - prev_south > 1 {
                    sides += 1;
                }
                prev_south = x;
            }
        }

        plots.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let mut prev_x = 0;
        let mut prev_west = -2;
        let mut prev_east = -2;

        for &(x, y) in plots.iter() {
            if x != prev_x {
                prev_west = -2;
                prev_east = -2;
                prev_x = x;
            }

            if !self.has_plot(x - 1, y) {
                if y - prev_west > 1 {
                    sides += 1;
                }
                prev_west = y;
            }

            if !self.has_plot(x + 1, y) {
                if y - prev_east > 1 {
                    sides += 1;
                }
                prev_east = y;
            }
        }

        sides
    }
}

pub fn part1(input: &str) -> u64 {
    collect_regions(input)
        .par_iter()
        .map(|region| region.area() * region.perimeter())
        .sum()
}

pub fn part2(input: &str) -> u64 {
    collect_regions(input)
        .par_iter()
        .map(|region| region.area() * region.sides())
        .sum()
}

fn collect_regions(input: &str) -> Vec<Region> {
    let input = input.trim_end().as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap() as i32;
    let height = input.len() as i32 / width;

    fn collect_i(
        input: &[u8],
        visited: &mut [u8],
        region: &mut Region,
        line_len: i32,
        max_x: i32,
        max_y: i32,
        byte: u8,
        x: i32,
        y: i32,
    ) {
        let next_index = r_index(x, y, line_len) as usize;

        if input[next_index] != byte {
            return;
        }

        if visited[next_index] != 0 {
            return;
        }

        region.add_plot(x, y);
        visited[next_index] = 1;

        collect(input, visited, region, line_len, max_x, max_y, byte, x, y);
    }

    fn collect(
        input: &[u8],
        visited: &mut [u8],
        region: &mut Region,
        line_len: i32,
        max_x: i32,
        max_y: i32,
        byte: u8,
        x: i32,
        y: i32,
    ) {
        if x > 0 {
            collect_i(
                input,
                visited,
                region,
                line_len,
                max_x,
                max_y,
                byte,
                x - 1,
                y,
            );
        }
        if x < max_x {
            collect_i(
                input,
                visited,
                region,
                line_len,
                max_x,
                max_y,
                byte,
                x + 1,
                y,
            );
        }
        if y > 0 {
            collect_i(
                input,
                visited,
                region,
                line_len,
                max_x,
                max_y,
                byte,
                x,
                y - 1,
            );
        }
        if y < max_y {
            collect_i(
                input,
                visited,
                region,
                line_len,
                max_x,
                max_y,
                byte,
                x,
                y + 1,
            );
        }
    }

    let mut regions = Vec::with_capacity(100);
    let mut visited = vec![0; input.len()];
    let mut i = 0;

    let line_len = width + 1;
    let max_x = width - 1;
    let max_y = height - 1;

    for y in 0..height {
        for x in 0..width {
            if visited[i] != 0 {
                i += 1;
                continue;
            }
            visited[i] = 1;
            let mut region = Region::new(width, height);
            region.add_plot(x, y);
            collect(
                &input,
                &mut visited,
                &mut region,
                line_len,
                max_x,
                max_y,
                input[i],
                x,
                y,
            );
            regions.push(region);
            i += 1;
        }
        i += 1;
    }

    regions
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const TEST_INPUT1: &str = "AAAA
BBCD
BBCC
EEEC";
    const TEST_INPUT2: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    const TEST_INPUT3: &str = "AA
AB";

    #[test]
    fn test_day12_part1() {
        assert_eq!(part1(TEST_INPUT1), 140);
        assert_eq!(part1(TEST_INPUT2), 1930);
        assert_eq!(part1(TEST_INPUT3), 28);
        let prod_input = read_to_string("./inputs/12.txt").unwrap();
        let prod_output = read_to_string("./outputs/12p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(part2(TEST_INPUT1), 80);
        assert_eq!(part2(TEST_INPUT2), 1206);
        assert_eq!(part2(TEST_INPUT3), 19);
        let prod_input = read_to_string("./inputs/12.txt").unwrap();
        let prod_output = read_to_string("./outputs/12p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
