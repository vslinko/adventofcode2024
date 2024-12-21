use std::simd::prelude::*;

const fn numeric_keypad_index(from: u8, to: u8) -> usize {
    // 18 is minimum multiplier that provides a unique index for each pair of buttons
    (from as usize) - 48 + ((to as usize) - 48) * 18
}

const LUT_SIZE: usize = numeric_keypad_index(b'A', b'A') + 1;

const LUT1: [usize; LUT_SIZE] = {
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

const LUT2: [usize; LUT_SIZE] = {
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

pub fn solve(input: &str, lut: &[usize]) -> usize {
    let input = input.as_bytes();

    let u1 = usizex8::splat(1);
    let u10 = usizex8::splat(10);
    let u18 = usizex8::splat(18);
    let u48 = usizex8::splat(48); // b'0' == 48
    let a_buttons = usizex8::splat(65); // b'A' == 65
    let u100 = usizex8::splat(100);
    let u5328 = usizex8::splat(5328); // 100 * 48 + 10 * 48 + 48

    let first_buttons_idxs = usizex8::from_array([0, 5, 10, 15, 20, 25, 30, 35]);
    let second_buttons_idxs = first_buttons_idxs + u1;
    let third_buttons_idxs = second_buttons_idxs + u1;

    let first_buttons = u8x8::gather_or_default(&input, first_buttons_idxs);
    let second_buttons = u8x8::gather_or_default(&input, second_buttons_idxs);
    let third_buttons = u8x8::gather_or_default(&input, third_buttons_idxs);

    let first_buttons = first_buttons.cast();
    let second_buttons = second_buttons.cast();
    let third_buttons = third_buttons.cast();

    let num = first_buttons * u100 + second_buttons * u10 + third_buttons - u5328;

    let idxs1 = a_buttons - u48 + (first_buttons - u48) * u18;
    let idxs2 = first_buttons - u48 + (second_buttons - u48) * u18;
    let idxs3 = second_buttons - u48 + (third_buttons - u48) * u18;
    let idxs4 = third_buttons - u48 + (a_buttons - u48) * u18;

    let moves1 = usizex8::gather_or_default(lut, idxs1);
    let moves2 = usizex8::gather_or_default(lut, idxs2);
    let moves3 = usizex8::gather_or_default(lut, idxs3);
    let moves4 = usizex8::gather_or_default(lut, idxs4);

    (num * (moves1 + moves2 + moves3 + moves4)).reduce_sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, &LUT1)
}

pub fn part2(input: &str) -> usize {
    solve(input, &LUT2)
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
