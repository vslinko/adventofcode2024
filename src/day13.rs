pub fn part1(input: &str) -> u64 {
    unsafe { inner(input, 0.0) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner(input, 10000000000000.0) }
}

unsafe fn inner(input: &str, increment: f64) -> u64 {
    let input = input.as_bytes();
    let mut i = 0;
    let mut sum = 0.0;

    while i < input.len() {
        i += 12; // skip "Button A: X+"

        let mut ax = *input.get_unchecked(i) as f64 * 10.0;
        i += 1;
        ax += *input.get_unchecked(i) as f64;
        ax -= 528.0; // 528 = b'0' * 10 + b'0'

        i += 5; // skip digit and ", Y+"

        let mut ay = *input.get_unchecked(i) as f64 * 10.0;
        i += 1;
        ay += *input.get_unchecked(i) as f64;
        ay -= 528.0; // 528 = b'0' * 10 + b'0'

        i += 14; // skip digit and "\nButton B: X+"

        let mut bx = *input.get_unchecked(i) as f64 * 10.0;
        i += 1;
        bx += *input.get_unchecked(i) as f64;
        bx -= 528.0; // 528 = b'0' * 10 + b'0'

        i += 5; // skip digit and ", Y+"

        let mut by = *input.get_unchecked(i) as f64 * 10.0;
        i += 1;
        by += *input.get_unchecked(i) as f64;
        by -= 528.0; // 528 = b'0' * 10 + b'0'

        i += 11; // skip digit and "\nPrize: X="

        let mut px = 0.0;
        loop {
            match input.get_unchecked(i) {
                b'0'..=b'9' => {
                    px = px * 10.0 + (input.get_unchecked(i) - b'0') as f64;
                    i += 1;
                }
                _ => {
                    break;
                }
            }
        }

        i += 4; // skip ", Y="

        let mut py = 0.0;
        loop {
            match input.get_unchecked(i) {
                b'0'..=b'9' => {
                    py = py * 10.0 + (input.get_unchecked(i) - b'0') as f64;
                    i += 1;
                }
                _ => {
                    break;
                }
            }
        }

        i += 2; // skip "\n\n"

        sum += smallest_number_of_tokens(ax, ay, bx, by, px + increment, py + increment);
    }

    sum as u64
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
