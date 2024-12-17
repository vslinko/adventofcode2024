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

fn eval_programm(mut a: i64, mut b: i64, mut c: i64, ops: &[i64]) -> String {
    let mut i = 0;
    let mut output = String::new();

    macro_rules! combo {
        ($i:expr) => {{
            match ops[$i] {
                0..=3 => ops[$i],
                4 => a,
                5 => b,
                6 => c,
                _ => 0,
            }
        }};
    }

    while i < ops.len() {
        let mut next_i = i + 2;

        match ops[i] {
            0 => {
                a /= 2_i64.pow(combo!(i + 1) as u32);
            }
            1 => {
                b ^= ops[i + 1];
            }
            2 => {
                b = combo!(i + 1) % 8;
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
                output.push_str(match combo!(i + 1) % 8 {
                    0 => "0,",
                    1 => "1,",
                    2 => "2,",
                    3 => "3,",
                    4 => "4,",
                    5 => "5,",
                    6 => "6,",
                    7 => "7,",
                    _ => "_,",
                });
            }
            6 => {
                b = a / 2_i64.pow(combo!(i + 1) as u32);
            }
            7 => {
                c = a / 2_i64.pow(combo!(i + 1) as u32);
            }
            _ => {}
        }

        i = next_i;
    }

    output.pop();
    output
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

    macro_rules! combo {
        ($i:expr) => {{
            match ops[$i] {
                0..=3 => ops[$i],
                4 => a,
                5 => b,
                6 => c,
                _ => 0,
            }
        }};
    }

    while i < ops.len() {
        let mut next_i = i + 2;

        match ops[i] {
            0 => {
                a /= 2_i64.pow(combo!(i + 1) as u32);
            }
            1 => {
                b ^= ops[i + 1];
            }
            2 => {
                b = combo!(i + 1) % 8;
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
                return combo!(i + 1) % 8;
            }
            6 => {
                b = a / 2_i64.pow(combo!(i + 1) as u32);
            }
            7 => {
                c = a / 2_i64.pow(combo!(i + 1) as u32);
            }
            _ => {}
        }

        i = next_i;
    }

    i64::MAX
}

macro_rules! make_eval {
    ($name:ident; $($instruction:expr, $operand:expr),*; $last:expr) => {
        unsafe fn $name(mut a: i64, mut b: i64, mut c: i64, ops: &[i64]) -> i64 {
            macro_rules! combo {
                ($a:expr) => {{
                    match ops[$a] {
                        0..=3 => ops[$a],
                        4 => a,
                        5 => b,
                        6 => c,
                        _ => 0,
                    }
                }};
            }

            macro_rules! make_step {
                ($a:expr, $b:expr) => {
                    match ops[$a] {
                        0 => {
                            a /= 2_i64.pow(combo!($b) as u32);
                        }
                        1 => {
                            b ^= ops[$b];
                        }
                        2 => {
                            b = combo!($b) % 8;
                        }
                        4 => {
                            b ^= c;
                        }
                        6 => {
                            b = a / 2_i64.pow(combo!($b) as u32);
                        }
                        7 => {
                            c = a / 2_i64.pow(combo!($b) as u32);
                        }
                        _ => {}
                    }
                };
            }

            $(
                make_step!($instruction, $operand);
            )*

            return combo!($last) % 8;
        }
    };
}

make_eval!(eval1; 0, 1; 3);
make_eval!(eval2; 0, 1, 2, 3; 5);
make_eval!(eval3; 0, 1, 2, 3, 4, 5; 7);
make_eval!(eval4; 0, 1, 2, 3, 4, 5, 6, 7; 9);
make_eval!(eval5; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9; 11);
make_eval!(eval6; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11; 13);
make_eval!(eval7; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13; 15);
make_eval!(eval8; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15; 17);
make_eval!(eval9; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17; 19);

unsafe fn find(
    eval_fn: unsafe fn(i64, i64, i64, &[i64]) -> i64,
    ops: &[i64],
    a: i64,
    b: i64,
    c: i64,
    depth: usize,
) -> i64 {
    if depth == ops.len() {
        return a;
    }

    let a8 = 8 * a;

    macro_rules! check {
        ($($e:expr)?) => {
            let a = a8$( + $e )?;

            if eval_fn(a, b, c, ops) == ops[ops.len() - 1 - depth] {
                let found = find(eval_fn, ops, a, b, c, depth + 1);

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

    let mut ops_before_ouput = 0;
    for i in (0..ops.len()).step_by(2) {
        if ops[i] == 5 {
            ops_before_ouput = i / 2;
            break;
        }
    }

    let eval_fn = match ops_before_ouput {
        1 => eval1,
        2 => eval2,
        3 => eval3,
        4 => eval4,
        5 => eval5,
        6 => eval6,
        7 => eval7,
        8 => eval8,
        9 => eval9,
        _ => eval_programm_first,
    };

    find(eval_fn, &ops, 0, b, c, 0)
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
