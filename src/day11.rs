use std::collections::HashMap;

pub fn part1(input: &str) -> u64 {
    calculate(input, 25)
}

pub fn part2(input: &str) -> u64 {
    calculate(input, 75)
}

fn calculate(input: &str, iters: u8) -> u64 {
    let mut map =
        input
            .trim_end()
            .split_whitespace()
            .fold(HashMap::new(), |mut acc, number_string| {
                acc.insert(number_string.to_string(), 1);
                acc
            });

    for _ in 0..iters {
        let mut new_map: HashMap<String, u64> = HashMap::new();

        for (number_string, count) in map.iter() {
            if *number_string == "0" {
                *new_map.entry("1".to_string()).or_insert(0) += count;
            } else if number_string.len() % 2 == 0 {
                let mid = number_string.len() / 2;
                let left = &number_string[..mid];
                let mut right = &number_string[mid..];
                while right.starts_with("0") && right.len() > 1 {
                    right = &right[1..];
                }

                *new_map.entry(left.to_string()).or_insert(0) += count;
                *new_map.entry(right.to_string()).or_insert(0) += count;
            } else {
                let num = number_string.parse::<u64>().unwrap() * 2024;
                *new_map.entry(num.to_string()).or_insert(0) += count;
            }
        }

        map = new_map;
    }

    map.values().sum()
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
