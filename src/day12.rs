const POSSIBLE_MOVES: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn part1(input: &str) -> u64 {
    let regions = collect_regions(input);

    let mut solution = 0;

    for region in regions {
        let area = region.len() as u64;
        let perimeter = calc_perimeter(&region);

        solution += area * perimeter;
    }

    return solution;
}

pub fn part2(input: &str) -> u64 {
    let regions = collect_regions(input);

    let mut solution = 0;

    for region in regions {
        let area = region.len() as u64;
        let sides = calc_sides(&region);

        solution += area * sides;
    }

    return solution;
}

fn calc_perimeter(region: &[(i32, i32)]) -> u64 {
    let mut perimeter = 0;

    for (x, y) in region {
        for &(dx, dy) in &POSSIBLE_MOVES {
            let next_x = x + dx;
            let next_y = y + dy;

            if region
                .iter()
                .all(|(plot_x, plot_y)| *plot_x != next_x || *plot_y != next_y)
            {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn calc_sides(region: &[(i32, i32)]) -> u64 {
    let mut sides = 0;

    let (from_x, till_x) = region.iter().fold((i32::MAX, 0), |(min_x, max_x), (x, _)| {
        (min_x.min(*x), max_x.max(*x))
    });
    let (from_y, till_y) = region.iter().fold((i32::MAX, 0), |(min_y, max_y), (_, y)| {
        (min_y.min(*y), max_y.max(*y))
    });

    let has_plot = |x, y| {
        region
            .iter()
            .any(|(plot_x, plot_y)| *plot_x == x && *plot_y == y)
    };

    for y in from_y..=till_y {
        let mut north = Vec::with_capacity((till_x - from_x + 1) as usize);
        let mut south = Vec::with_capacity((till_x - from_x + 1) as usize);

        for x in from_x..=till_x {
            if has_plot(x, y) {
                if !has_plot(x, y - 1) {
                    north.push(x);
                }
                if !has_plot(x, y + 1) {
                    south.push(x);
                }
            }
        }

        if !north.is_empty() {
            let mut prev = north[0];
            for &x in &north[1..] {
                if x - prev > 1 {
                    sides += 1;
                }
                prev = x;
            }
            sides += 1;
        }

        if !south.is_empty() {
            let mut prev = south[0];
            for &x in &south[1..] {
                if x - prev > 1 {
                    sides += 1;
                }
                prev = x;
            }
            sides += 1;
        }
    }

    for x in from_x..=till_x {
        let mut west = Vec::with_capacity((till_y - from_y + 1) as usize);
        let mut east = Vec::with_capacity((till_y - from_y + 1) as usize);

        for y in from_y..=till_y {
            if has_plot(x, y) {
                if !has_plot(x - 1, y) {
                    west.push(y);
                }
                if !has_plot(x + 1, y) {
                    east.push(y);
                }
            }
        }

        if !west.is_empty() {
            let mut prev = west[0];
            for &x in &west[1..] {
                if x - prev > 1 {
                    sides += 1;
                }
                prev = x;
            }
            sides += 1;
        }

        if !east.is_empty() {
            let mut prev = east[0];
            for &x in &east[1..] {
                if x - prev > 1 {
                    sides += 1;
                }
                prev = x;
            }
            sides += 1;
        }
    }

    sides
}

fn collect_regions(input: &str) -> Vec<Vec<(i32, i32)>> {
    let input = input.trim_end().as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap() as i32;
    let height = input.len() as i32 / width;

    fn get_index(x: i32, y: i32, width: i32) -> usize {
        (y * (width + 1) + x) as usize
    }

    fn collect(
        input: &[u8],
        visited: &mut [u8],
        region: &mut Vec<(i32, i32)>,
        width: i32,
        height: i32,
        byte: u8,
        x: i32,
        y: i32,
    ) {
        for &(dx, dy) in &POSSIBLE_MOVES {
            let next_x = x + dx;
            let next_y = y + dy;

            if next_x < 0 || next_x >= width || next_y < 0 || next_y >= height {
                continue;
            }

            let next_index = get_index(next_x, next_y, width);

            if input[next_index] != byte {
                continue;
            }

            if visited[next_index] != 0 {
                continue;
            }

            region.push((next_x, next_y));
            visited[next_index] = 1;

            collect(input, visited, region, width, height, byte, next_x, next_y);
        }
    }

    let mut regions = Vec::with_capacity(100);
    let mut visited = vec![0; input.len()];
    let mut i = 0;

    for y in 0..height {
        for x in 0..width {
            if visited[i] != 0 {
                i += 1;
                continue;
            }
            visited[i] = 1;
            let mut region = Vec::with_capacity(100);
            region.push((x, y));
            collect(
                &input,
                &mut visited,
                &mut region,
                width,
                height,
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

    #[test]
    fn test_day12_part1() {
        assert_eq!(part1(TEST_INPUT1), 140);
        assert_eq!(part1(TEST_INPUT2), 1930);
        let prod_input = read_to_string("./inputs/12.txt").unwrap();
        let prod_output = read_to_string("./outputs/12p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(part2(TEST_INPUT1), 80);
        assert_eq!(part2(TEST_INPUT2), 1206);
        let prod_input = read_to_string("./inputs/12.txt").unwrap();
        let prod_output = read_to_string("./outputs/12p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
