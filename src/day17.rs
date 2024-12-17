use std::fmt::Display;

macro_rules! read_unsigned {
    ($input:expr, $i:expr) => {{
        let mut num = 0;
        loop {
            match $input.get_unchecked($i) {
                b'0'..=b'9' => {
                    num = num * 10 + ($input.get_unchecked($i) - b'0') as u64;
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
            0..=3 => *$operand,
            4 => $a,
            5 => $b,
            6 => $c,
            _ => 0,
        }
    }};
}

macro_rules! adv {
    ($operand:expr, $a:expr, $b:expr, $c:expr) => {
        $a /= 2_u64.pow(combo!($operand, $a, $b, $c) as u32)
    };
}

macro_rules! bxl {
    ($operand:expr, $a:expr, $b:expr, $c:expr) => {
        $b ^= $operand
    };
}

macro_rules! bst {
    ($operand:expr, $a:expr, $b:expr, $c:expr) => {
        $b = combo!($operand, $a, $b, $c) % 8
    };
}

macro_rules! jnz {
    ($next_i:expr, $operand:expr, $a:expr, $b:expr, $c:expr) => {
        if $a != 0 {
            $next_i = $operand as usize;
        }
    };
}

macro_rules! bxc {
    ($operand:expr, $a:expr, $b:expr, $c:expr) => {
        $b ^= $c
    };
}

macro_rules! out {
    ($operand:expr, $a:expr, $b:expr, $c:expr) => {
        combo!($operand, $a, $b, $c) % 8
    };
}

macro_rules! bdv {
    ($operand:expr, $a:expr, $b:expr, $c:expr) => {
        $b = $a / 2_u64.pow(combo!($operand, $a, $b, $c) as u32)
    };
}

macro_rules! cdv {
    ($operand:expr, $a:expr, $b:expr, $c:expr) => {
        $c = $a / 2_u64.pow(combo!($operand, $a, $b, $c) as u32)
    };
}

unsafe fn eval_full(mut a: u64, mut b: u64, mut c: u64, ops: &[u64]) -> String {
    let mut i = 0;
    let mut output = String::new();

    while i < ops.len() {
        let mut next_i = i + 2;

        match ops.get_unchecked(i) {
            0 => adv!(ops.get_unchecked(i + 1), a, b, c),
            1 => bxl!(ops.get_unchecked(i + 1), a, b, c),
            2 => bst!(ops.get_unchecked(i + 1), a, b, c),
            3 => jnz!(next_i, *ops.get_unchecked(i + 1), a, b, c),
            4 => bxc!(ops.get_unchecked(i + 1), a, b, c),
            5 => {
                output.push_str(match out!(ops.get_unchecked(i + 1), a, b, c) {
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
            6 => bdv!(ops.get_unchecked(i + 1), a, b, c),
            7 => cdv!(ops.get_unchecked(i + 1), a, b, c),
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

    eval_full(a, b, c, &ops)
}

pub fn part2(input: &str) -> impl Display {
    unsafe { inner2(input) }
}

unsafe fn eval_return_first(mut a: u64, mut b: u64, mut c: u64, ops: &[u64]) -> u64 {
    let mut i = 0;

    while i < ops.len() {
        let mut next_i = i + 2;

        match ops.get_unchecked(i) {
            0 => adv!(ops.get_unchecked(i + 1), a, b, c),
            1 => bxl!(ops.get_unchecked(i + 1), a, b, c),
            2 => bst!(ops.get_unchecked(i + 1), a, b, c),
            3 => jnz!(next_i, *ops.get_unchecked(i + 1), a, b, c),
            4 => bxc!(ops.get_unchecked(i + 1), a, b, c),
            5 => return out!(ops.get_unchecked(i + 1), a, b, c),
            6 => bdv!(ops.get_unchecked(i + 1), a, b, c),
            7 => cdv!(ops.get_unchecked(i + 1), a, b, c),
            _ => {}
        }

        i = next_i;
    }

    u64::MAX
}

macro_rules! make_eval {
    ($name:ident; $($instruction:expr, $operand:expr),*; $last:expr) => {
        unsafe fn $name(mut a: u64, mut b: u64, mut c: u64, ops: &[u64]) -> u64 {
            macro_rules! make_step {
                ($a:expr, $b:expr) => {
                    match ops.get_unchecked($a) {
                        0 => adv!(ops.get_unchecked($b), a, b, c),
                        1 => bxl!(ops.get_unchecked($b), a, b, c),
                        2 => bst!(ops.get_unchecked($b), a, b, c),
                        4 => bxc!(ops.get_unchecked($b), a, b, c),
                        6 => bdv!(ops.get_unchecked($b), a, b, c),
                        7 => cdv!(ops.get_unchecked($b), a, b, c),
                        _ => {}
                    }
                };
            }

            $(
                make_step!($instruction, $operand);
            )*

            return out!(ops.get_unchecked($last), a, b, c);
        }
    };
}

make_eval!(eval_return_first1; 0, 1; 3);
make_eval!(eval_return_first2; 0, 1, 2, 3; 5);
make_eval!(eval_return_first3; 0, 1, 2, 3, 4, 5; 7);
make_eval!(eval_return_first4; 0, 1, 2, 3, 4, 5, 6, 7; 9);
make_eval!(eval_return_first5; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9; 11);
make_eval!(eval_return_first6; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11; 13);
make_eval!(eval_return_first7; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13; 15);
make_eval!(eval_return_first8; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15; 17);
make_eval!(eval_return_first9; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17; 19);

unsafe fn find(
    eval_fn: unsafe fn(u64, u64, u64, &[u64]) -> u64,
    ops: &[u64],
    a: u64,
    b: u64,
    c: u64,
    depth: usize,
) -> u64 {
    if depth == ops.len() {
        return a;
    }

    let a8 = 8 * a;
    let expected = *ops.get_unchecked(ops.len() - 1 - depth);

    macro_rules! check {
        ($a:expr) => {
            let a = $a;

            if eval_fn(a, b, c, ops) == expected {
                let found = find(eval_fn, ops, a, b, c, depth + 1);

                if found != 0 {
                    return found;
                }
            }
        };
    }

    check!(a8);
    check!(a8 + 1);
    check!(a8 + 2);
    check!(a8 + 3);
    check!(a8 + 4);
    check!(a8 + 5);
    check!(a8 + 6);
    check!(a8 + 7);

    0
}

unsafe fn inner2(input: &str) -> u64 {
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

    let mut instuctions_before_output = 0;
    for i in (0..ops.len()).step_by(2) {
        match *ops.get_unchecked(i) {
            3 => break, // found loop before first output
            5 => {
                instuctions_before_output = i / 2;
                break;
            }
            _ => {}
        }
    }

    let eval_fn = match instuctions_before_output {
        1 => eval_return_first1,
        2 => eval_return_first2,
        3 => eval_return_first3,
        4 => eval_return_first4,
        5 => eval_return_first5,
        6 => eval_return_first6,
        7 => eval_return_first7,
        8 => eval_return_first8,
        9 => eval_return_first9,
        _ => eval_return_first,
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
