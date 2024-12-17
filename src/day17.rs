use std::fmt::Display;

macro_rules! read_unsigned {
    ($input:expr, $i:expr) => {{
        let mut num = 0;
        loop {
            match $input.get_unchecked($i) {
                b'0'..=b'9' => {
                    num = num * 10 + ($input.get_unchecked($i) - b'0') as i64;
                    $i += 1;
                }
                _ => {
                    break;
                }
            }
        }
        num
    }};
}

macro_rules! combo {
    ($operand:expr, $a:expr, $b:expr, $c:expr) => {{
        match $operand {
            0..=3 => $operand,
            4 => $a,
            5 => $b,
            6 => $c,
            _ => 0,
        }
    }};
}

fn eval_programm(mut a: i64, mut b: i64, mut c: i64, ops: &[i64]) -> String {
    let mut i = 0;
    let mut output = Vec::with_capacity(20);

    while i < ops.len() {
        let mut next_i = i + 2;

        match ops[i] {
            0 => {
                a /= 2_i64.pow(combo!(ops[i + 1], a, b, c) as u32);
            }
            1 => {
                b ^= ops[i + 1];
            }
            2 => {
                b = combo!(ops[i + 1], a, b, c) % 8;
            }
            3 => {
                if a != 0 {
                    next_i = ops[i + 1] as usize;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                output.push(match combo!(ops[i + 1], a, b, c) % 8 {
                    0 => "0",
                    1 => "1",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    5 => "5",
                    6 => "6",
                    7 => "7",
                    _ => "_",
                });
            }
            6 => {
                b = a / 2_i64.pow(combo!(ops[i + 1], a, b, c) as u32);
            }
            7 => {
                c = a / 2_i64.pow(combo!(ops[i + 1], a, b, c) as u32);
            }
            _ => {}
        }

        i = next_i;
    }

    output.join(",")
}

pub fn part1(input: &str) -> impl Display {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> impl Display {
    let input = input.as_bytes();
    let mut i = 12; // skip "Register A: "
    let a = read_unsigned!(input, i);
    i += 13; // skip "\nRegister B: "
    let b = read_unsigned!(input, i);
    i += 13; // skip "\nRegister C: "
    let c = read_unsigned!(input, i);
    i += 11; // skip "\n\nProgram: "
    let mut ops = Vec::with_capacity(20);
    while i < input.len() {
        match input.get_unchecked(i) {
            b'0' => ops.push(0),
            b'1' => ops.push(1),
            b'2' => ops.push(2),
            b'3' => ops.push(3),
            b'4' => ops.push(4),
            b'5' => ops.push(5),
            b'6' => ops.push(6),
            b'7' => ops.push(7),
            _ => {}
        };
        match input.get_unchecked(i + 2) {
            b'0' => ops.push(0),
            b'1' => ops.push(1),
            b'2' => ops.push(2),
            b'3' => ops.push(3),
            b'4' => ops.push(4),
            b'5' => ops.push(5),
            b'6' => ops.push(6),
            b'7' => ops.push(7),
            _ => {}
        };
        i += 4;
    }

    eval_programm(a, b, c, &ops)
}

pub fn part2(input: &str) -> impl Display {
    unsafe { inner2(input) }
}

fn eval_programm_first(mut a: i64, mut b: i64, mut c: i64, ops: &[i64]) -> i64 {
    let mut i = 0;

    while i < ops.len() {
        let mut next_i = i + 2;

        match ops[i] {
            0 => {
                a /= 2_i64.pow(combo!(ops[i + 1], a, b, c) as u32);
            }
            1 => {
                b ^= ops[i + 1];
            }
            2 => {
                b = combo!(ops[i + 1], a, b, c) % 8;
            }
            3 => {
                if a != 0 {
                    next_i = ops[i + 1] as usize;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                return combo!(ops[i + 1], a, b, c) % 8;
            }
            6 => {
                b = a / 2_i64.pow(combo!(ops[i + 1], a, b, c) as u32);
            }
            7 => {
                c = a / 2_i64.pow(combo!(ops[i + 1], a, b, c) as u32);
            }
            _ => {}
        }

        i = next_i;
    }

    i64::MAX
}

fn find(ops: &[i64], a: i64, b: i64, c: i64, depth: usize) -> i64 {
    if depth == ops.len() {
        return a;
    }

    let a8 = 8 * a;

    macro_rules! check {
        ($($e:expr)?) => {
            let a = a8$( + $e )?;

            if eval_programm_first(a, b, c, ops) == ops[ops.len() - 1 - depth] {
                let found = find(ops, a, b, c, depth + 1);

                if found != 0 {
                    return found;
                }
            }
        };
    }

    check!();
    check!(1);
    check!(2);
    check!(3);
    check!(4);
    check!(5);
    check!(6);
    check!(7);

    0
}

unsafe fn inner2(input: &str) -> i64 {
    let input = input.as_bytes();
    let mut i = 12; // skip "Register A: "
    loop {
        match input.get_unchecked(i) {
            b'0'..=b'9' => {
                i += 1;
            }
            _ => {
                break;
            }
        }
    }
    i += 13; // skip "\nRegister B: "
    let b = read_unsigned!(input, i);
    i += 13; // skip "\nRegister C: "
    let c = read_unsigned!(input, i);
    i += 11; // skip "\n\nProgram: "
    let mut ops = Vec::with_capacity(20);
    while i < input.len() {
        match *input.get_unchecked(i) {
            b'0' => ops.push(0),
            b'1' => ops.push(1),
            b'2' => ops.push(2),
            b'3' => ops.push(3),
            b'4' => ops.push(4),
            b'5' => ops.push(5),
            b'6' => ops.push(6),
            b'7' => ops.push(7),
            _ => {}
        };
        match *input.get_unchecked(i + 2) {
            b'0' => ops.push(0),
            b'1' => ops.push(1),
            b'2' => ops.push(2),
            b'3' => ops.push(3),
            b'4' => ops.push(4),
            b'5' => ops.push(5),
            b'6' => ops.push(6),
            b'7' => ops.push(7),
            _ => {}
        };
        i += 4;
    }

    find(&ops, 0, b, c, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day17_part1() {
        let prod_input = read_to_string("./inputs/17.txt").unwrap();
        let prod_output = read_to_string("./outputs/17p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day17_part2() {
        let prod_input = read_to_string("./inputs/17.txt").unwrap();
        let prod_output = read_to_string("./outputs/17p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
