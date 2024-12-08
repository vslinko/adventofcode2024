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

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 4);
    }
}
