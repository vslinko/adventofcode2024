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
    use std::fs::read_to_string;

    #[test]
    fn test_day3_part1() {
        let prod_input = read_to_string("./inputs/3.txt").unwrap();
        let prod_output = read_to_string("./outputs/3p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day3_part2() {
        let prod_input = read_to_string("./inputs/3.txt").unwrap();
        let prod_output = read_to_string("./outputs/3p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
