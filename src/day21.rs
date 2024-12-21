use std::mem::transmute;

const LUT1: [u64; 1000] = unsafe { transmute(*include_bytes!("../luts/day21_1.bin")) };
const LUT2: [u64; 1000] = unsafe { transmute(*include_bytes!("../luts/day21_2.bin")) };

unsafe fn solve(input: &[u8], lut: &[u64; 1000]) -> u64 {
    macro_rules! solve {
        ($a:expr, $b:expr, $c:expr) => {
            lut.get_unchecked(
                (*input.get_unchecked($a) as usize) * 100
                    + (*input.get_unchecked($b) as usize) * 10
                    + (*input.get_unchecked($c) as usize)
                    - 5328,
            )
        };
    }

    solve!(0, 1, 2) + solve!(5, 6, 7) + solve!(10, 11, 12) + solve!(15, 16, 17) + solve!(20, 21, 22)
}

pub fn part1(input: &str) -> u64 {
    unsafe { solve(input.as_bytes(), &LUT1) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { solve(input.as_bytes(), &LUT2) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day21_part1() {
        let prod_input = read_to_string("./inputs/21.txt").unwrap();
        let prod_output = read_to_string("./outputs/21p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day21_part2() {
        let prod_input = read_to_string("./inputs/21.txt").unwrap();
        let prod_output = read_to_string("./outputs/21p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
