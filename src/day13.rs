macro_rules! read2 {
    ($input:expr, $i:expr) => {{
        let mut num = *$input.get_unchecked($i) as f64 * 10.0;
        num += *$input.get_unchecked($i + 1) as f64;
        num - 528.0 // 528 = b'0' * 10 + b'0'
    }};
}

macro_rules! read {
    ($input:expr, $i:expr) => {{
        let mut num = 0.0;
        loop {
            match $input.get_unchecked($i) {
                b'0'..=b'9' => {
                    num = num * 10.0 + ($input.get_unchecked($i) - b'0') as f64;
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

macro_rules! solution {
    ( $input:expr $(, $delta:expr )? ) => {
        unsafe {
            let input = $input.as_bytes();
            let mut i = 0;
            let mut sum = 0.0;

            while i < input.len() {
                i += 12; // skip "Button A: X+"
                let ax = read2!(input, i);
                i += 6; // skip number and ", Y+"
                let ay = read2!(input, i);
                i += 15; // skip number and "\nButton B: X+"
                let bx = read2!(input, i);
                i += 6; // skip number and ", Y+"
                let by = read2!(input, i);
                i += 12; // skip number and "\nPrize: X="
                let px = read!(input, i);
                i += 4; // skip ", Y="
                let py = read!(input, i);
                i += 2; // skip "\n\n"

                sum += smallest_number_of_tokens(ax, ay, bx, by, px $( + $delta )?, py $( + $delta )?);
            }

            sum as u64
        }
    };
}

pub fn part1(input: &str) -> u64 {
    solution!(input)
}

pub fn part2(input: &str) -> u64 {
    solution!(input, 10000000000000.0)
}

fn smallest_number_of_tokens(ax: f64, ay: f64, bx: f64, by: f64, px: f64, py: f64) -> f64 {
    let m2 = by / bx;
    let ix = (py - m2 * px) / (ay / ax - m2);

    let a_count = (ix / ax).round();
    let b_count = ((px - ix) / bx).round();

    if ax * a_count + bx * b_count != px || ay * a_count + by * b_count != py {
        return 0.0;
    }

    a_count * 3.0 + b_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_day13_part1() {
        assert_eq!(part1(TEST_INPUT).to_string(), "480");
        let prod_input = read_to_string("./inputs/13.txt").unwrap();
        let prod_output = read_to_string("./outputs/13p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day13_part2() {
        assert_eq!(part2(TEST_INPUT).to_string(), "875318608908");
        let prod_input = read_to_string("./inputs/13.txt").unwrap();
        let prod_output = read_to_string("./outputs/13p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
