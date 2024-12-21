const fn numeric_keypad_index(from: u8, to: u8) -> usize {
    (from as usize) - 48 + ((to as usize) - 48) * 18
}

const MOVES1: [u64; 324] = {
    let mut moves = [0; 324];

    moves[numeric_keypad_index(48, 48)] = 1;
    moves[numeric_keypad_index(48, 49)] = 25;
    moves[numeric_keypad_index(48, 50)] = 12;
    moves[numeric_keypad_index(48, 51)] = 19;
    moves[numeric_keypad_index(48, 52)] = 26;
    moves[numeric_keypad_index(48, 53)] = 13;
    moves[numeric_keypad_index(48, 54)] = 20;
    moves[numeric_keypad_index(48, 55)] = 27;
    moves[numeric_keypad_index(48, 56)] = 14;
    moves[numeric_keypad_index(48, 57)] = 21;
    moves[numeric_keypad_index(48, 65)] = 10;
    moves[numeric_keypad_index(49, 48)] = 21;
    moves[numeric_keypad_index(49, 49)] = 1;
    moves[numeric_keypad_index(49, 50)] = 10;
    moves[numeric_keypad_index(49, 51)] = 11;
    moves[numeric_keypad_index(49, 52)] = 12;
    moves[numeric_keypad_index(49, 53)] = 19;
    moves[numeric_keypad_index(49, 54)] = 20;
    moves[numeric_keypad_index(49, 55)] = 13;
    moves[numeric_keypad_index(49, 56)] = 20;
    moves[numeric_keypad_index(49, 57)] = 21;
    moves[numeric_keypad_index(49, 65)] = 22;
    moves[numeric_keypad_index(50, 48)] = 16;
    moves[numeric_keypad_index(50, 49)] = 18;
    moves[numeric_keypad_index(50, 50)] = 1;
    moves[numeric_keypad_index(50, 51)] = 10;
    moves[numeric_keypad_index(50, 52)] = 21;
    moves[numeric_keypad_index(50, 53)] = 12;
    moves[numeric_keypad_index(50, 54)] = 19;
    moves[numeric_keypad_index(50, 55)] = 22;
    moves[numeric_keypad_index(50, 56)] = 13;
    moves[numeric_keypad_index(50, 57)] = 20;
    moves[numeric_keypad_index(50, 65)] = 17;
    moves[numeric_keypad_index(51, 48)] = 21;
    moves[numeric_keypad_index(51, 49)] = 19;
    moves[numeric_keypad_index(51, 50)] = 18;
    moves[numeric_keypad_index(51, 51)] = 1;
    moves[numeric_keypad_index(51, 52)] = 22;
    moves[numeric_keypad_index(51, 53)] = 21;
    moves[numeric_keypad_index(51, 54)] = 12;
    moves[numeric_keypad_index(51, 55)] = 23;
    moves[numeric_keypad_index(51, 56)] = 22;
    moves[numeric_keypad_index(51, 57)] = 13;
    moves[numeric_keypad_index(51, 65)] = 16;
    moves[numeric_keypad_index(52, 48)] = 22;
    moves[numeric_keypad_index(52, 49)] = 16;
    moves[numeric_keypad_index(52, 50)] = 17;
    moves[numeric_keypad_index(52, 51)] = 18;
    moves[numeric_keypad_index(52, 52)] = 1;
    moves[numeric_keypad_index(52, 53)] = 10;
    moves[numeric_keypad_index(52, 54)] = 11;
    moves[numeric_keypad_index(52, 55)] = 12;
    moves[numeric_keypad_index(52, 56)] = 19;
    moves[numeric_keypad_index(52, 57)] = 20;
    moves[numeric_keypad_index(52, 65)] = 23;
    moves[numeric_keypad_index(53, 48)] = 17;
    moves[numeric_keypad_index(53, 49)] = 21;
    moves[numeric_keypad_index(53, 50)] = 16;
    moves[numeric_keypad_index(53, 51)] = 17;
    moves[numeric_keypad_index(53, 52)] = 18;
    moves[numeric_keypad_index(53, 53)] = 1;
    moves[numeric_keypad_index(53, 54)] = 10;
    moves[numeric_keypad_index(53, 55)] = 21;
    moves[numeric_keypad_index(53, 56)] = 12;
    moves[numeric_keypad_index(53, 57)] = 19;
    moves[numeric_keypad_index(53, 65)] = 18;
    moves[numeric_keypad_index(54, 48)] = 22;
    moves[numeric_keypad_index(54, 49)] = 22;
    moves[numeric_keypad_index(54, 50)] = 21;
    moves[numeric_keypad_index(54, 51)] = 16;
    moves[numeric_keypad_index(54, 52)] = 19;
    moves[numeric_keypad_index(54, 53)] = 18;
    moves[numeric_keypad_index(54, 54)] = 1;
    moves[numeric_keypad_index(54, 55)] = 22;
    moves[numeric_keypad_index(54, 56)] = 21;
    moves[numeric_keypad_index(54, 57)] = 12;
    moves[numeric_keypad_index(54, 65)] = 17;
    moves[numeric_keypad_index(55, 48)] = 23;
    moves[numeric_keypad_index(55, 49)] = 17;
    moves[numeric_keypad_index(55, 50)] = 18;
    moves[numeric_keypad_index(55, 51)] = 19;
    moves[numeric_keypad_index(55, 52)] = 16;
    moves[numeric_keypad_index(55, 53)] = 17;
    moves[numeric_keypad_index(55, 54)] = 18;
    moves[numeric_keypad_index(55, 55)] = 1;
    moves[numeric_keypad_index(55, 56)] = 10;
    moves[numeric_keypad_index(55, 57)] = 11;
    moves[numeric_keypad_index(55, 65)] = 24;
    moves[numeric_keypad_index(56, 48)] = 18;
    moves[numeric_keypad_index(56, 49)] = 22;
    moves[numeric_keypad_index(56, 50)] = 17;
    moves[numeric_keypad_index(56, 51)] = 18;
    moves[numeric_keypad_index(56, 52)] = 21;
    moves[numeric_keypad_index(56, 53)] = 16;
    moves[numeric_keypad_index(56, 54)] = 17;
    moves[numeric_keypad_index(56, 55)] = 18;
    moves[numeric_keypad_index(56, 56)] = 1;
    moves[numeric_keypad_index(56, 57)] = 10;
    moves[numeric_keypad_index(56, 65)] = 19;
    moves[numeric_keypad_index(57, 48)] = 23;
    moves[numeric_keypad_index(57, 49)] = 23;
    moves[numeric_keypad_index(57, 50)] = 22;
    moves[numeric_keypad_index(57, 51)] = 17;
    moves[numeric_keypad_index(57, 52)] = 22;
    moves[numeric_keypad_index(57, 53)] = 21;
    moves[numeric_keypad_index(57, 54)] = 16;
    moves[numeric_keypad_index(57, 55)] = 19;
    moves[numeric_keypad_index(57, 56)] = 18;
    moves[numeric_keypad_index(57, 57)] = 1;
    moves[numeric_keypad_index(57, 65)] = 18;
    moves[numeric_keypad_index(65, 48)] = 18;
    moves[numeric_keypad_index(65, 49)] = 26;
    moves[numeric_keypad_index(65, 50)] = 21;
    moves[numeric_keypad_index(65, 51)] = 12;
    moves[numeric_keypad_index(65, 52)] = 27;
    moves[numeric_keypad_index(65, 53)] = 22;
    moves[numeric_keypad_index(65, 54)] = 13;
    moves[numeric_keypad_index(65, 55)] = 28;
    moves[numeric_keypad_index(65, 56)] = 23;
    moves[numeric_keypad_index(65, 57)] = 14;
    moves[numeric_keypad_index(65, 65)] = 1;

    moves
};

const MOVES2: [u64; 324] = {
    let mut moves = [0; 324];

    moves[numeric_keypad_index(48, 48)] = 1;
    moves[numeric_keypad_index(48, 49)] = 31420065369;
    moves[numeric_keypad_index(48, 50)] = 14752615084;
    moves[numeric_keypad_index(48, 51)] = 24095973437;
    moves[numeric_keypad_index(48, 52)] = 31420065370;
    moves[numeric_keypad_index(48, 53)] = 14752615085;
    moves[numeric_keypad_index(48, 54)] = 24095973438;
    moves[numeric_keypad_index(48, 55)] = 31420065371;
    moves[numeric_keypad_index(48, 56)] = 14752615086;
    moves[numeric_keypad_index(48, 57)] = 24095973439;
    moves[numeric_keypad_index(48, 65)] = 14287938116;
    moves[numeric_keypad_index(49, 48)] = 27052881363;
    moves[numeric_keypad_index(49, 49)] = 1;
    moves[numeric_keypad_index(49, 50)] = 14287938116;
    moves[numeric_keypad_index(49, 51)] = 14287938117;
    moves[numeric_keypad_index(49, 52)] = 14752615084;
    moves[numeric_keypad_index(49, 53)] = 24095973437;
    moves[numeric_keypad_index(49, 54)] = 24095973438;
    moves[numeric_keypad_index(49, 55)] = 14752615085;
    moves[numeric_keypad_index(49, 56)] = 24095973438;
    moves[numeric_keypad_index(49, 57)] = 24095973439;
    moves[numeric_keypad_index(49, 65)] = 27052881364;
    moves[numeric_keypad_index(50, 48)] = 20790420654;
    moves[numeric_keypad_index(50, 49)] = 22411052532;
    moves[numeric_keypad_index(50, 50)] = 1;
    moves[numeric_keypad_index(50, 51)] = 14287938116;
    moves[numeric_keypad_index(50, 52)] = 28154654777;
    moves[numeric_keypad_index(50, 53)] = 14752615084;
    moves[numeric_keypad_index(50, 54)] = 24095973437;
    moves[numeric_keypad_index(50, 55)] = 28154654778;
    moves[numeric_keypad_index(50, 56)] = 14752615085;
    moves[numeric_keypad_index(50, 57)] = 24095973438;
    moves[numeric_keypad_index(50, 65)] = 22778092491;
    moves[numeric_keypad_index(51, 48)] = 27622800565;
    moves[numeric_keypad_index(51, 49)] = 22411052533;
    moves[numeric_keypad_index(51, 50)] = 22411052532;
    moves[numeric_keypad_index(51, 51)] = 1;
    moves[numeric_keypad_index(51, 52)] = 28154654778;
    moves[numeric_keypad_index(51, 53)] = 28154654777;
    moves[numeric_keypad_index(51, 54)] = 14752615084;
    moves[numeric_keypad_index(51, 55)] = 28154654779;
    moves[numeric_keypad_index(51, 56)] = 28154654778;
    moves[numeric_keypad_index(51, 57)] = 14752615085;
    moves[numeric_keypad_index(51, 65)] = 20790420654;
    moves[numeric_keypad_index(52, 48)] = 27052881364;
    moves[numeric_keypad_index(52, 49)] = 20790420654;
    moves[numeric_keypad_index(52, 50)] = 22778092491;
    moves[numeric_keypad_index(52, 51)] = 22778092492;
    moves[numeric_keypad_index(52, 52)] = 1;
    moves[numeric_keypad_index(52, 53)] = 14287938116;
    moves[numeric_keypad_index(52, 54)] = 14287938117;
    moves[numeric_keypad_index(52, 55)] = 14752615084;
    moves[numeric_keypad_index(52, 56)] = 24095973437;
    moves[numeric_keypad_index(52, 57)] = 24095973438;
    moves[numeric_keypad_index(52, 65)] = 27052881365;
    moves[numeric_keypad_index(53, 48)] = 20790420655;
    moves[numeric_keypad_index(53, 49)] = 27622800565;
    moves[numeric_keypad_index(53, 50)] = 20790420654;
    moves[numeric_keypad_index(53, 51)] = 22778092491;
    moves[numeric_keypad_index(53, 52)] = 22411052532;
    moves[numeric_keypad_index(53, 53)] = 1;
    moves[numeric_keypad_index(53, 54)] = 14287938116;
    moves[numeric_keypad_index(53, 55)] = 28154654777;
    moves[numeric_keypad_index(53, 56)] = 14752615084;
    moves[numeric_keypad_index(53, 57)] = 24095973437;
    moves[numeric_keypad_index(53, 65)] = 22778092492;
    moves[numeric_keypad_index(54, 48)] = 27622800566;
    moves[numeric_keypad_index(54, 49)] = 27622800566;
    moves[numeric_keypad_index(54, 50)] = 27622800565;
    moves[numeric_keypad_index(54, 51)] = 20790420654;
    moves[numeric_keypad_index(54, 52)] = 22411052533;
    moves[numeric_keypad_index(54, 53)] = 22411052532;
    moves[numeric_keypad_index(54, 54)] = 1;
    moves[numeric_keypad_index(54, 55)] = 28154654778;
    moves[numeric_keypad_index(54, 56)] = 28154654777;
    moves[numeric_keypad_index(54, 57)] = 14752615084;
    moves[numeric_keypad_index(54, 65)] = 20790420655;
    moves[numeric_keypad_index(55, 48)] = 27052881365;
    moves[numeric_keypad_index(55, 49)] = 20790420655;
    moves[numeric_keypad_index(55, 50)] = 22778092492;
    moves[numeric_keypad_index(55, 51)] = 22778092493;
    moves[numeric_keypad_index(55, 52)] = 20790420654;
    moves[numeric_keypad_index(55, 53)] = 22778092491;
    moves[numeric_keypad_index(55, 54)] = 22778092492;
    moves[numeric_keypad_index(55, 55)] = 1;
    moves[numeric_keypad_index(55, 56)] = 14287938116;
    moves[numeric_keypad_index(55, 57)] = 14287938117;
    moves[numeric_keypad_index(55, 65)] = 27052881366;
    moves[numeric_keypad_index(56, 48)] = 20790420656;
    moves[numeric_keypad_index(56, 49)] = 27622800566;
    moves[numeric_keypad_index(56, 50)] = 20790420655;
    moves[numeric_keypad_index(56, 51)] = 22778092492;
    moves[numeric_keypad_index(56, 52)] = 27622800565;
    moves[numeric_keypad_index(56, 53)] = 20790420654;
    moves[numeric_keypad_index(56, 54)] = 22778092491;
    moves[numeric_keypad_index(56, 55)] = 22411052532;
    moves[numeric_keypad_index(56, 56)] = 1;
    moves[numeric_keypad_index(56, 57)] = 14287938116;
    moves[numeric_keypad_index(56, 65)] = 22778092493;
    moves[numeric_keypad_index(57, 48)] = 27622800567;
    moves[numeric_keypad_index(57, 49)] = 27622800567;
    moves[numeric_keypad_index(57, 50)] = 27622800566;
    moves[numeric_keypad_index(57, 51)] = 20790420655;
    moves[numeric_keypad_index(57, 52)] = 27622800566;
    moves[numeric_keypad_index(57, 53)] = 27622800565;
    moves[numeric_keypad_index(57, 54)] = 20790420654;
    moves[numeric_keypad_index(57, 55)] = 22411052533;
    moves[numeric_keypad_index(57, 56)] = 22411052532;
    moves[numeric_keypad_index(57, 57)] = 1;
    moves[numeric_keypad_index(57, 65)] = 20790420656;
    moves[numeric_keypad_index(65, 48)] = 22411052532;
    moves[numeric_keypad_index(65, 49)] = 31420065370;
    moves[numeric_keypad_index(65, 50)] = 28154654777;
    moves[numeric_keypad_index(65, 51)] = 14752615084;
    moves[numeric_keypad_index(65, 52)] = 31420065371;
    moves[numeric_keypad_index(65, 53)] = 28154654778;
    moves[numeric_keypad_index(65, 54)] = 14752615085;
    moves[numeric_keypad_index(65, 55)] = 31420065372;
    moves[numeric_keypad_index(65, 56)] = 28154654779;
    moves[numeric_keypad_index(65, 57)] = 14752615086;
    moves[numeric_keypad_index(65, 65)] = 1;

    moves
};

unsafe fn parse_num(n: &[u8]) -> u64 {
    (*n.get_unchecked(0) as u64) * 100
        + (*n.get_unchecked(1) as u64) * 10
        + (*n.get_unchecked(2) as u64)
        - 5328
}

unsafe fn solve(input: &str, moves: &[u64; 324]) -> u64 {
    input
        .lines()
        .map(|code| {
            let mut pressed = 0;
            let mut from_numpad_button = b'A';

            for &to_numpad_button in code.as_bytes() {
                let min_buttons_to_press =
                    moves[numeric_keypad_index(from_numpad_button, to_numpad_button)];

                pressed += min_buttons_to_press;
                from_numpad_button = to_numpad_button;
            }

            pressed * parse_num(code.as_bytes())
        })
        .sum()
}

pub fn part1(input: &str) -> u64 {
    unsafe { solve(input, &MOVES1) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { solve(input, &MOVES2) }
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
