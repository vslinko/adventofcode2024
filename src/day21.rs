use std::mem::transmute;

const LUT1: [u64; 1000] = unsafe { transmute(*include_bytes!("../luts/day21_1.bin")) };
const LUT2: [u64; 1000] = unsafe { transmute(*include_bytes!("../luts/day21_2.bin")) };

#[rustfmt::skip]
unsafe fn parse_num(n: &[u8]) -> usize {
    (*n.get_unchecked(0) as usize) * 100 +
    (*n.get_unchecked(1) as usize) * 10 +
    (*n.get_unchecked(2) as usize) - 5328
}

pub fn part1(input: &str) -> u64 {
    unsafe {
        input
            .lines()
            .map(|code| LUT1.get_unchecked(parse_num(code.as_bytes())))
            .sum()
    }
}

pub fn part2(input: &str) -> u64 {
    unsafe {
        input
            .lines()
            .map(|code| LUT2.get_unchecked(parse_num(code.as_bytes())))
            .sum()
    }
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
