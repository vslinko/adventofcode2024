use rustc_hash::FxHashMap;

pub fn part1(input: &str) -> u64 {
    calculate(input, 25)
}

pub fn part2(input: &str) -> u64 {
    calculate(input, 75)
}

fn calculate(input: &str, iters: u8) -> u64 {
    let mut left_map =
        input
            .trim_end()
            .split(' ')
            .fold(FxHashMap::default(), |mut acc, number_string| {
                acc.insert(number_string.parse::<u64>().unwrap(), 1);
                acc
            });

    let mut right_map = FxHashMap::default();

    for _ in 0..iters {
        for (number, count) in left_map.iter() {
            if *number == 0 {
                *right_map.entry(1).or_default() += count;
                continue;
            }

            let len = number.ilog10() + 1;

            if len % 2 == 1 {
                *right_map.entry(number * 2024).or_default() += count;
                continue;
            }

            let pow = 10u64.pow(len / 2);

            *right_map.entry(number / pow).or_default() += count;
            *right_map.entry(number % pow).or_default() += count;
        }

        (left_map, right_map) = (right_map, left_map);
        right_map.clear();
    }

    left_map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_day11_part1() {
        assert_eq!(part1(TEST_INPUT), 55312);
        let prod_input = read_to_string("./inputs/11.txt").unwrap();
        let prod_output = read_to_string("./outputs/11p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day11_part2() {
        assert_eq!(part2(TEST_INPUT), 65601038650482);
        let prod_input = read_to_string("./inputs/11.txt").unwrap();
        let prod_output = read_to_string("./outputs/11p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
