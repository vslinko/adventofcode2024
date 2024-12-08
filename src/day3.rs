use regex::Regex;
use std::collections::BTreeMap;

#[derive(Debug)]
enum Action {
    Do,
    Dont,
    Mul(i32, i32),
}

fn perform(actions: &Vec<&Action>) -> i32 {
    let mut result = 0;
    let mut enabled = true;

    for action in actions {
        match action {
            Action::Mul(a, b) => {
                if enabled {
                    result += a * b;
                }
            }
            Action::Do => {
                enabled = true;
            }
            Action::Dont => {
                enabled = false;
            }
        }
    }

    result
}

pub fn part1(input: &str) -> i32 {
    let mut actions: BTreeMap<usize, Action> = BTreeMap::new();

    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for cap in mul_re.captures_iter(input) {
        let index = cap.get(0).unwrap().start();
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();

        actions.insert(index, Action::Mul(a, b));
    }

    perform(&actions.values().collect())
}

pub fn part2(input: &str) -> i32 {
    let mut actions: BTreeMap<usize, Action> = BTreeMap::new();

    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for cap in mul_re.captures_iter(input) {
        let index = cap.get(0).unwrap().start();
        let a: i32 = cap[1].parse().unwrap();
        let b: i32 = cap[2].parse().unwrap();

        actions.insert(index, Action::Mul(a, b));
    }

    let do_re = Regex::new(r"do(n't)?\(\)").unwrap();
    for cap in do_re.captures_iter(input) {
        let index = cap.get(0).unwrap().start();

        if cap.get(0).unwrap().as_str() == "do()" {
            actions.insert(index, Action::Do);
        } else {
            actions.insert(index, Action::Dont);
        }
    }

    perform(&actions.values().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        let result = part1(INPUT1);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT2);
        assert_eq!(result, 48);
    }
}
