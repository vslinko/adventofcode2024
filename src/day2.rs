fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .fold(Vec::with_capacity(1000), |mut acc, line| {
            let nums = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();

            acc.push(nums);
            acc
        })
}

pub fn part1(input: &str) -> i32 {
    let reports = parse(input);

    reports.iter().fold(0, |safe_reports, report| {
        if is_valid_report(&report) {
            safe_reports + 1
        } else {
            safe_reports
        }
    })
}
fn is_valid_report(report: &[i32]) -> bool {
    assert!(report.len() >= 2);

    let sign = (report[1] - report[0]).signum();

    (1..report.len()).all(|i| {
        let diff = report[i] - report[i - 1];
        let abs_diff = diff.abs();

        diff.signum() == sign && abs_diff >= 1 && abs_diff <= 3
    })
}

pub fn part2(input: &str) -> i32 {
    let reports = parse(input);

    reports.iter().fold(0, |safe_reports, report| {
        if is_valid_report_with_problem_dampener(&report) {
            safe_reports + 1
        } else {
            safe_reports
        }
    })
}

fn is_valid_report_with_problem_dampener(report: &Vec<i32>) -> bool {
    assert!(report.len() >= 2);

    if is_valid_report(&report) {
        return true;
    }

    (0..report.len()).any(|i| {
        let mut report = report.clone();
        report.remove(i);
        is_valid_report(&report)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day2_part1() {
        let prod_input = read_to_string("./inputs/2.txt").unwrap();
        let prod_output = read_to_string("./outputs/2p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day2_part2() {
        let prod_input = read_to_string("./inputs/2.txt").unwrap();
        let prod_output = read_to_string("./outputs/2p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
