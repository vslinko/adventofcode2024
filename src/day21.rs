const fn numeric_keypad_index(from: u8, to: u8) -> usize {
    // 18 is minimum multiple that creates a unique index for each pair of buttons
    (from as usize) - 48 + ((to as usize) - 48) * 18
}

const LUT_SIZE: usize = numeric_keypad_index(b'A', b'A') + 1;

const LUT1: [u64; LUT_SIZE] = {
    let mut lut = [0; LUT_SIZE];

    lut[numeric_keypad_index(b'0', b'0')] = 1;
    lut[numeric_keypad_index(b'0', b'1')] = 25;
    lut[numeric_keypad_index(b'0', b'2')] = 12;
    lut[numeric_keypad_index(b'0', b'3')] = 19;
    lut[numeric_keypad_index(b'0', b'4')] = 26;
    lut[numeric_keypad_index(b'0', b'5')] = 13;
    lut[numeric_keypad_index(b'0', b'6')] = 20;
    lut[numeric_keypad_index(b'0', b'7')] = 27;
    lut[numeric_keypad_index(b'0', b'8')] = 14;
    lut[numeric_keypad_index(b'0', b'9')] = 21;
    lut[numeric_keypad_index(b'0', b'A')] = 10;
    lut[numeric_keypad_index(b'1', b'0')] = 21;
    lut[numeric_keypad_index(b'1', b'1')] = 1;
    lut[numeric_keypad_index(b'1', b'2')] = 10;
    lut[numeric_keypad_index(b'1', b'3')] = 11;
    lut[numeric_keypad_index(b'1', b'4')] = 12;
    lut[numeric_keypad_index(b'1', b'5')] = 19;
    lut[numeric_keypad_index(b'1', b'6')] = 20;
    lut[numeric_keypad_index(b'1', b'7')] = 13;
    lut[numeric_keypad_index(b'1', b'8')] = 20;
    lut[numeric_keypad_index(b'1', b'9')] = 21;
    lut[numeric_keypad_index(b'1', b'A')] = 22;
    lut[numeric_keypad_index(b'2', b'0')] = 16;
    lut[numeric_keypad_index(b'2', b'1')] = 18;
    lut[numeric_keypad_index(b'2', b'2')] = 1;
    lut[numeric_keypad_index(b'2', b'3')] = 10;
    lut[numeric_keypad_index(b'2', b'4')] = 21;
    lut[numeric_keypad_index(b'2', b'5')] = 12;
    lut[numeric_keypad_index(b'2', b'6')] = 19;
    lut[numeric_keypad_index(b'2', b'7')] = 22;
    lut[numeric_keypad_index(b'2', b'8')] = 13;
    lut[numeric_keypad_index(b'2', b'9')] = 20;
    lut[numeric_keypad_index(b'2', b'A')] = 17;
    lut[numeric_keypad_index(b'3', b'0')] = 21;
    lut[numeric_keypad_index(b'3', b'1')] = 19;
    lut[numeric_keypad_index(b'3', b'2')] = 18;
    lut[numeric_keypad_index(b'3', b'3')] = 1;
    lut[numeric_keypad_index(b'3', b'4')] = 22;
    lut[numeric_keypad_index(b'3', b'5')] = 21;
    lut[numeric_keypad_index(b'3', b'6')] = 12;
    lut[numeric_keypad_index(b'3', b'7')] = 23;
    lut[numeric_keypad_index(b'3', b'8')] = 22;
    lut[numeric_keypad_index(b'3', b'9')] = 13;
    lut[numeric_keypad_index(b'3', b'A')] = 16;
    lut[numeric_keypad_index(b'4', b'0')] = 22;
    lut[numeric_keypad_index(b'4', b'1')] = 16;
    lut[numeric_keypad_index(b'4', b'2')] = 17;
    lut[numeric_keypad_index(b'4', b'3')] = 18;
    lut[numeric_keypad_index(b'4', b'4')] = 1;
    lut[numeric_keypad_index(b'4', b'5')] = 10;
    lut[numeric_keypad_index(b'4', b'6')] = 11;
    lut[numeric_keypad_index(b'4', b'7')] = 12;
    lut[numeric_keypad_index(b'4', b'8')] = 19;
    lut[numeric_keypad_index(b'4', b'9')] = 20;
    lut[numeric_keypad_index(b'4', b'A')] = 23;
    lut[numeric_keypad_index(b'5', b'0')] = 17;
    lut[numeric_keypad_index(b'5', b'1')] = 21;
    lut[numeric_keypad_index(b'5', b'2')] = 16;
    lut[numeric_keypad_index(b'5', b'3')] = 17;
    lut[numeric_keypad_index(b'5', b'4')] = 18;
    lut[numeric_keypad_index(b'5', b'5')] = 1;
    lut[numeric_keypad_index(b'5', b'6')] = 10;
    lut[numeric_keypad_index(b'5', b'7')] = 21;
    lut[numeric_keypad_index(b'5', b'8')] = 12;
    lut[numeric_keypad_index(b'5', b'9')] = 19;
    lut[numeric_keypad_index(b'5', b'A')] = 18;
    lut[numeric_keypad_index(b'6', b'0')] = 22;
    lut[numeric_keypad_index(b'6', b'1')] = 22;
    lut[numeric_keypad_index(b'6', b'2')] = 21;
    lut[numeric_keypad_index(b'6', b'3')] = 16;
    lut[numeric_keypad_index(b'6', b'4')] = 19;
    lut[numeric_keypad_index(b'6', b'5')] = 18;
    lut[numeric_keypad_index(b'6', b'6')] = 1;
    lut[numeric_keypad_index(b'6', b'7')] = 22;
    lut[numeric_keypad_index(b'6', b'8')] = 21;
    lut[numeric_keypad_index(b'6', b'9')] = 12;
    lut[numeric_keypad_index(b'6', b'A')] = 17;
    lut[numeric_keypad_index(b'7', b'0')] = 23;
    lut[numeric_keypad_index(b'7', b'1')] = 17;
    lut[numeric_keypad_index(b'7', b'2')] = 18;
    lut[numeric_keypad_index(b'7', b'3')] = 19;
    lut[numeric_keypad_index(b'7', b'4')] = 16;
    lut[numeric_keypad_index(b'7', b'5')] = 17;
    lut[numeric_keypad_index(b'7', b'6')] = 18;
    lut[numeric_keypad_index(b'7', b'7')] = 1;
    lut[numeric_keypad_index(b'7', b'8')] = 10;
    lut[numeric_keypad_index(b'7', b'9')] = 11;
    lut[numeric_keypad_index(b'7', b'A')] = 24;
    lut[numeric_keypad_index(b'8', b'0')] = 18;
    lut[numeric_keypad_index(b'8', b'1')] = 22;
    lut[numeric_keypad_index(b'8', b'2')] = 17;
    lut[numeric_keypad_index(b'8', b'3')] = 18;
    lut[numeric_keypad_index(b'8', b'4')] = 21;
    lut[numeric_keypad_index(b'8', b'5')] = 16;
    lut[numeric_keypad_index(b'8', b'6')] = 17;
    lut[numeric_keypad_index(b'8', b'7')] = 18;
    lut[numeric_keypad_index(b'8', b'8')] = 1;
    lut[numeric_keypad_index(b'8', b'9')] = 10;
    lut[numeric_keypad_index(b'8', b'A')] = 19;
    lut[numeric_keypad_index(b'9', b'0')] = 23;
    lut[numeric_keypad_index(b'9', b'1')] = 23;
    lut[numeric_keypad_index(b'9', b'2')] = 22;
    lut[numeric_keypad_index(b'9', b'3')] = 17;
    lut[numeric_keypad_index(b'9', b'4')] = 22;
    lut[numeric_keypad_index(b'9', b'5')] = 21;
    lut[numeric_keypad_index(b'9', b'6')] = 16;
    lut[numeric_keypad_index(b'9', b'7')] = 19;
    lut[numeric_keypad_index(b'9', b'8')] = 18;
    lut[numeric_keypad_index(b'9', b'9')] = 1;
    lut[numeric_keypad_index(b'9', b'A')] = 18;
    lut[numeric_keypad_index(b'A', b'0')] = 18;
    lut[numeric_keypad_index(b'A', b'1')] = 26;
    lut[numeric_keypad_index(b'A', b'2')] = 21;
    lut[numeric_keypad_index(b'A', b'3')] = 12;
    lut[numeric_keypad_index(b'A', b'4')] = 27;
    lut[numeric_keypad_index(b'A', b'5')] = 22;
    lut[numeric_keypad_index(b'A', b'6')] = 13;
    lut[numeric_keypad_index(b'A', b'7')] = 28;
    lut[numeric_keypad_index(b'A', b'8')] = 23;
    lut[numeric_keypad_index(b'A', b'9')] = 14;
    lut[numeric_keypad_index(b'A', b'A')] = 1;

    lut
};

const LUT2: [u64; LUT_SIZE] = {
    let mut lut = [0; LUT_SIZE];

    lut[numeric_keypad_index(b'0', b'0')] = 1;
    lut[numeric_keypad_index(b'0', b'1')] = 31420065369;
    lut[numeric_keypad_index(b'0', b'2')] = 14752615084;
    lut[numeric_keypad_index(b'0', b'3')] = 24095973437;
    lut[numeric_keypad_index(b'0', b'4')] = 31420065370;
    lut[numeric_keypad_index(b'0', b'5')] = 14752615085;
    lut[numeric_keypad_index(b'0', b'6')] = 24095973438;
    lut[numeric_keypad_index(b'0', b'7')] = 31420065371;
    lut[numeric_keypad_index(b'0', b'8')] = 14752615086;
    lut[numeric_keypad_index(b'0', b'9')] = 24095973439;
    lut[numeric_keypad_index(b'0', b'A')] = 14287938116;
    lut[numeric_keypad_index(b'1', b'0')] = 27052881363;
    lut[numeric_keypad_index(b'1', b'1')] = 1;
    lut[numeric_keypad_index(b'1', b'2')] = 14287938116;
    lut[numeric_keypad_index(b'1', b'3')] = 14287938117;
    lut[numeric_keypad_index(b'1', b'4')] = 14752615084;
    lut[numeric_keypad_index(b'1', b'5')] = 24095973437;
    lut[numeric_keypad_index(b'1', b'6')] = 24095973438;
    lut[numeric_keypad_index(b'1', b'7')] = 14752615085;
    lut[numeric_keypad_index(b'1', b'8')] = 24095973438;
    lut[numeric_keypad_index(b'1', b'9')] = 24095973439;
    lut[numeric_keypad_index(b'1', b'A')] = 27052881364;
    lut[numeric_keypad_index(b'2', b'0')] = 20790420654;
    lut[numeric_keypad_index(b'2', b'1')] = 22411052532;
    lut[numeric_keypad_index(b'2', b'2')] = 1;
    lut[numeric_keypad_index(b'2', b'3')] = 14287938116;
    lut[numeric_keypad_index(b'2', b'4')] = 28154654777;
    lut[numeric_keypad_index(b'2', b'5')] = 14752615084;
    lut[numeric_keypad_index(b'2', b'6')] = 24095973437;
    lut[numeric_keypad_index(b'2', b'7')] = 28154654778;
    lut[numeric_keypad_index(b'2', b'8')] = 14752615085;
    lut[numeric_keypad_index(b'2', b'9')] = 24095973438;
    lut[numeric_keypad_index(b'2', b'A')] = 22778092491;
    lut[numeric_keypad_index(b'3', b'0')] = 27622800565;
    lut[numeric_keypad_index(b'3', b'1')] = 22411052533;
    lut[numeric_keypad_index(b'3', b'2')] = 22411052532;
    lut[numeric_keypad_index(b'3', b'3')] = 1;
    lut[numeric_keypad_index(b'3', b'4')] = 28154654778;
    lut[numeric_keypad_index(b'3', b'5')] = 28154654777;
    lut[numeric_keypad_index(b'3', b'6')] = 14752615084;
    lut[numeric_keypad_index(b'3', b'7')] = 28154654779;
    lut[numeric_keypad_index(b'3', b'8')] = 28154654778;
    lut[numeric_keypad_index(b'3', b'9')] = 14752615085;
    lut[numeric_keypad_index(b'3', b'A')] = 20790420654;
    lut[numeric_keypad_index(b'4', b'0')] = 27052881364;
    lut[numeric_keypad_index(b'4', b'1')] = 20790420654;
    lut[numeric_keypad_index(b'4', b'2')] = 22778092491;
    lut[numeric_keypad_index(b'4', b'3')] = 22778092492;
    lut[numeric_keypad_index(b'4', b'4')] = 1;
    lut[numeric_keypad_index(b'4', b'5')] = 14287938116;
    lut[numeric_keypad_index(b'4', b'6')] = 14287938117;
    lut[numeric_keypad_index(b'4', b'7')] = 14752615084;
    lut[numeric_keypad_index(b'4', b'8')] = 24095973437;
    lut[numeric_keypad_index(b'4', b'9')] = 24095973438;
    lut[numeric_keypad_index(b'4', b'A')] = 27052881365;
    lut[numeric_keypad_index(b'5', b'0')] = 20790420655;
    lut[numeric_keypad_index(b'5', b'1')] = 27622800565;
    lut[numeric_keypad_index(b'5', b'2')] = 20790420654;
    lut[numeric_keypad_index(b'5', b'3')] = 22778092491;
    lut[numeric_keypad_index(b'5', b'4')] = 22411052532;
    lut[numeric_keypad_index(b'5', b'5')] = 1;
    lut[numeric_keypad_index(b'5', b'6')] = 14287938116;
    lut[numeric_keypad_index(b'5', b'7')] = 28154654777;
    lut[numeric_keypad_index(b'5', b'8')] = 14752615084;
    lut[numeric_keypad_index(b'5', b'9')] = 24095973437;
    lut[numeric_keypad_index(b'5', b'A')] = 22778092492;
    lut[numeric_keypad_index(b'6', b'0')] = 27622800566;
    lut[numeric_keypad_index(b'6', b'1')] = 27622800566;
    lut[numeric_keypad_index(b'6', b'2')] = 27622800565;
    lut[numeric_keypad_index(b'6', b'3')] = 20790420654;
    lut[numeric_keypad_index(b'6', b'4')] = 22411052533;
    lut[numeric_keypad_index(b'6', b'5')] = 22411052532;
    lut[numeric_keypad_index(b'6', b'6')] = 1;
    lut[numeric_keypad_index(b'6', b'7')] = 28154654778;
    lut[numeric_keypad_index(b'6', b'8')] = 28154654777;
    lut[numeric_keypad_index(b'6', b'9')] = 14752615084;
    lut[numeric_keypad_index(b'6', b'A')] = 20790420655;
    lut[numeric_keypad_index(b'7', b'0')] = 27052881365;
    lut[numeric_keypad_index(b'7', b'1')] = 20790420655;
    lut[numeric_keypad_index(b'7', b'2')] = 22778092492;
    lut[numeric_keypad_index(b'7', b'3')] = 22778092493;
    lut[numeric_keypad_index(b'7', b'4')] = 20790420654;
    lut[numeric_keypad_index(b'7', b'5')] = 22778092491;
    lut[numeric_keypad_index(b'7', b'6')] = 22778092492;
    lut[numeric_keypad_index(b'7', b'7')] = 1;
    lut[numeric_keypad_index(b'7', b'8')] = 14287938116;
    lut[numeric_keypad_index(b'7', b'9')] = 14287938117;
    lut[numeric_keypad_index(b'7', b'A')] = 27052881366;
    lut[numeric_keypad_index(b'8', b'0')] = 20790420656;
    lut[numeric_keypad_index(b'8', b'1')] = 27622800566;
    lut[numeric_keypad_index(b'8', b'2')] = 20790420655;
    lut[numeric_keypad_index(b'8', b'3')] = 22778092492;
    lut[numeric_keypad_index(b'8', b'4')] = 27622800565;
    lut[numeric_keypad_index(b'8', b'5')] = 20790420654;
    lut[numeric_keypad_index(b'8', b'6')] = 22778092491;
    lut[numeric_keypad_index(b'8', b'7')] = 22411052532;
    lut[numeric_keypad_index(b'8', b'8')] = 1;
    lut[numeric_keypad_index(b'8', b'9')] = 14287938116;
    lut[numeric_keypad_index(b'8', b'A')] = 22778092493;
    lut[numeric_keypad_index(b'9', b'0')] = 27622800567;
    lut[numeric_keypad_index(b'9', b'1')] = 27622800567;
    lut[numeric_keypad_index(b'9', b'2')] = 27622800566;
    lut[numeric_keypad_index(b'9', b'3')] = 20790420655;
    lut[numeric_keypad_index(b'9', b'4')] = 27622800566;
    lut[numeric_keypad_index(b'9', b'5')] = 27622800565;
    lut[numeric_keypad_index(b'9', b'6')] = 20790420654;
    lut[numeric_keypad_index(b'9', b'7')] = 22411052533;
    lut[numeric_keypad_index(b'9', b'8')] = 22411052532;
    lut[numeric_keypad_index(b'9', b'9')] = 1;
    lut[numeric_keypad_index(b'9', b'A')] = 20790420656;
    lut[numeric_keypad_index(b'A', b'0')] = 22411052532;
    lut[numeric_keypad_index(b'A', b'1')] = 31420065370;
    lut[numeric_keypad_index(b'A', b'2')] = 28154654777;
    lut[numeric_keypad_index(b'A', b'3')] = 14752615084;
    lut[numeric_keypad_index(b'A', b'4')] = 31420065371;
    lut[numeric_keypad_index(b'A', b'5')] = 28154654778;
    lut[numeric_keypad_index(b'A', b'6')] = 14752615085;
    lut[numeric_keypad_index(b'A', b'7')] = 31420065372;
    lut[numeric_keypad_index(b'A', b'8')] = 28154654779;
    lut[numeric_keypad_index(b'A', b'9')] = 14752615086;
    lut[numeric_keypad_index(b'A', b'A')] = 1;

    lut
};

unsafe fn parse_num(n: &[u8]) -> u64 {
    (*n.get_unchecked(0) as u64) * 100
        + (*n.get_unchecked(1) as u64) * 10
        + (*n.get_unchecked(2) as u64)
        - 5328
}

unsafe fn calculate_code(moves: &[u64; LUT_SIZE], code: &str) -> u64 {
    let mut pressed = 0;
    let mut from_numpad_button = b'A';

    for &to_numpad_button in code.as_bytes() {
        pressed += moves.get_unchecked(numeric_keypad_index(from_numpad_button, to_numpad_button));
        from_numpad_button = to_numpad_button;
    }

    pressed * parse_num(code.as_bytes())
}

pub fn part1(input: &str) -> u64 {
    unsafe { input.lines().map(|code| calculate_code(&LUT1, code)).sum() }
}

pub fn part2(input: &str) -> u64 {
    unsafe { input.lines().map(|code| calculate_code(&LUT2, code)).sum() }
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
