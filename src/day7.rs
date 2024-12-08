use rayon::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Operation {
    None,
    Add,
    Mul,
    Concat,
}

#[derive(Debug)]
enum Mode {
    Basic,    // Only Add and Mul operations
    Extended, // Add, Mul and Concat operations
}

fn check_equation(test_value: u64, numbers: &Vec<u64>, mode: Mode) -> bool {
    let n = numbers.len();
    let operations_len = n - 1;

    let mut i = 0;
    let mut results = vec![0u64; n];
    let mut operations = vec![Operation::None; operations_len];

    results[0] = numbers[0];

    let last_operation = match mode {
        Mode::Basic => Operation::Mul,
        Mode::Extended => Operation::Concat,
    };

    loop {
        if i == 0 && operations[i] == last_operation {
            return false;
        }

        if i == operations_len {
            if results[i] == test_value {
                return true;
            }
            i -= 1;
        }

        let next_i = i + 1;

        (operations[i], results[next_i], i) = match (&operations[i], &mode) {
            // none -> add
            (Operation::None, _) => (Operation::Add, results[i] + numbers[next_i], next_i),

            // add -> mul
            (Operation::Add, _) => (Operation::Mul, results[i] * numbers[next_i], next_i),

            // mul -> concat if Mode::Extended
            (Operation::Mul, Mode::Extended) => (
                Operation::Concat,
                concat(results[i], numbers[next_i]),
                next_i,
            ),

            // concat -> none if Mode::Extended
            // mul -> none if Mode::Basic
            (Operation::Mul, Mode::Basic) | (Operation::Concat, _) => {
                (Operation::None, results[next_i], i - 1)
            }
        };
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let col = line.find(':').unwrap();

            let test_value = line[0..col].parse::<u64>().unwrap();

            let numbers = line[col + 2..]
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            if check_equation(test_value, &numbers, Mode::Basic) {
                test_value
            } else {
                0
            }
        })
        .sum()
}

fn concat(a: u64, b: u64) -> u64 {
    let digits = (b as f64).log10().floor() as u32 + 1;
    a * 10_u64.pow(digits) + b
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .par_bridge()
        .map(|line| {
            let col = line.find(':').unwrap();

            let test_value = line[0..col].parse::<u64>().unwrap();

            let numbers = line[col + 2..]
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            if check_equation(test_value, &numbers, Mode::Extended) {
                test_value
            } else {
                0
            }
        })
        .sum()
}
