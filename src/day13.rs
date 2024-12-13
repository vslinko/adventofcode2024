type Case = (f64, f64, f64, f64, f64, f64);

pub fn part1(input: &str) -> u64 {
    parse(input)
        .map(|(ax, ay, bx, by, px, py)| solve(ax, ay, bx, by, px, py))
        .sum()
}

pub fn part2(input: &str) -> u64 {
    parse(input)
        .map(|(ax, ay, bx, by, px, py)| {
            solve(ax, ay, bx, by, px + 10000000000000.0, py + 10000000000000.0)
        })
        .sum()
}

fn solve(ax: f64, ay: f64, bx: f64, by: f64, px: f64, py: f64) -> u64 {
    let m1 = ay / ax;
    let m2 = by / bx;
    let b2 = py - m2 * px;
    let ix = b2 / (m1 - m2);

    let a_count = (ix / ax).round();
    let b_count = ((px - ix) / bx).round();

    if ax * a_count + bx * b_count != px || ay * a_count + by * b_count != py {
        return 0;
    }

    (a_count as u64) * 3 + b_count as u64
}

fn parse(input: &str) -> Parse {
    Parse {
        input: input.trim_end().as_bytes(),
        i: 0,
    }
}

struct Parse<'a> {
    i: usize,
    input: &'a [u8],
}

impl Parse<'_> {
    fn read2(&mut self) -> f64 {
        let mut num = self.input[self.i] as f64 * 10.0;
        self.i += 1;
        num += self.input[self.i] as f64;
        self.i += 1;
        num - 528.0 // 528 = b'0' * 10 + b'0'
    }

    fn read(&mut self) -> f64 {
        let mut num = 0.0;

        while self.i < self.input.len() {
            match self.input[self.i] {
                b'0'..=b'9' => {
                    num = num * 10.0 + (self.input[self.i] - b'0') as f64;
                    self.i += 1;
                }
                _ => {
                    break;
                }
            }
        }

        num
    }
}

impl Iterator for Parse<'_> {
    type Item = Case;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.input.len() {
            return None;
        }

        self.i += 12; // skip "Button A: X+"
        let ax = self.read2();
        self.i += 4; // skip ", Y+"
        let ay = self.read2();
        self.i += 13; // skip "\nButton B: X+"
        let bx = self.read2();
        self.i += 4; // skip ", Y+"
        let by = self.read2();
        self.i += 10; // skip "\nPrize: X="
        let px = self.read();
        self.i += 4; // skip ", Y="
        let py = self.read();
        self.i += 2; // skip "\n\n"

        Some((ax, ay, bx, by, px, py))
    }
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
Prize: X=18641, Y=10279";

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
