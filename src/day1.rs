use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse(&input);

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(a, b)| (a - b).abs()).sum()
}

pub fn part2(input: &str) -> i32 {
    let (left, right) = parse(&input);

    let right_counter = right
        .iter()
        .fold(HashMap::with_capacity(right.len()), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

    left.iter()
        .fold(0, |similarity, num| match right_counter.get(num) {
            Some(&count) => similarity + num * count,
            _ => similarity,
        })
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input.lines().enumerate().fold(
        (vec![0i32; 1000], vec![0i32; 1000]),
        |mut acc, (index, line)| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .take(2)
                .map(|s| s.parse().unwrap())
                .collect();

            acc.0[index] = nums[0];
            acc.1[index] = nums[1];

            acc
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day1_part1() {
        let prod_input = read_to_string("./inputs/1.txt").unwrap();
        let prod_output = read_to_string("./outputs/1p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day1_part2() {
        let prod_input = read_to_string("./inputs/1.txt").unwrap();
        let prod_output = read_to_string("./outputs/1p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
