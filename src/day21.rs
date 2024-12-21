use rustc_hash::{FxBuildHasher, FxHashMap};

fn get_numeric_keypad_paths(from: u8, to: u8) -> Vec<Vec<u8>> {
    match (from as u16) - 48 + ((to as u16) - 48) * 18 {
        0 | 19 | 38 | 57 | 76 | 95 | 114 | 133 | 152 | 171 | 323 => vec![vec![b'A']], // 00 11 22 33 44 55 66 77 88 99 AA
        18 => vec![vec![b'^', b'<', b'A']],                                           // 01
        36 | 73 | 92 | 111 | 130 | 149 | 168 | 71 => vec![vec![b'^', b'A']], // 02 14 25 36 47 58 69 A3
        54 | 91 | 110 | 148 | 167 => vec![vec![b'>', b'^', b'A'], vec![b'^', b'>', b'A']], // 03 15 26 48 59
        72 => vec![vec![b'^', b'^', b'<', b'A']],                                          // 04
        90 | 127 | 146 | 165 | 125 => vec![vec![b'^', b'^', b'A']], // 05 17 28 39 A6
        108 | 145 | 164 => vec![vec![b'>', b'^', b'^', b'A'], vec![b'^', b'^', b'>', b'A']], // 06 18 29
        126 => vec![vec![b'^', b'^', b'^', b'<', b'A']],                                     // 07
        144 | 179 => vec![vec![b'^', b'^', b'^', b'A']], // 08 A9
        162 => vec![
            vec![b'>', b'^', b'^', b'^', b'A'],
            vec![b'^', b'^', b'^', b'>', b'A'],
        ], // 09
        306 | 37 | 56 | 94 | 113 | 151 | 170 => vec![vec![b'>', b'A']], // 0A 12 23 45 56 78 89
        1 => vec![vec![b'>', b'v', b'A']],               // 10
        55 | 112 | 169 => vec![vec![b'>', b'>', b'A']],  // 13 46 79
        109 | 166 => vec![vec![b'>', b'>', b'^', b'A'], vec![b'^', b'>', b'>', b'A']], // 16 49
        163 => vec![
            vec![b'>', b'>', b'^', b'^', b'A'],
            vec![b'^', b'^', b'>', b'>', b'A'],
        ], // 19
        307 => vec![vec![b'>', b'>', b'v', b'A']],       // 1A
        2 | 309 | 22 | 41 | 60 | 79 | 98 | 117 => vec![vec![b'v', b'A']], // 20 3A 41 52 63 74 85 96
        20 | 39 | 77 | 96 | 134 | 153 | 17 => vec![vec![b'<', b'A']], // 21 32 54 65 87 98 A0
        74 | 93 | 131 | 150 | 53 => vec![vec![b'<', b'^', b'A'], vec![b'^', b'<', b'A']], // 24 35 57 68 A2
        128 | 147 | 107 => vec![vec![b'<', b'^', b'^', b'A'], vec![b'^', b'^', b'<', b'A']], // 27 38 A5
        308 | 40 | 59 | 97 | 116 => vec![vec![b'>', b'v', b'A'], vec![b'v', b'>', b'A']], // 2A 42 53 75 86
        3 | 23 | 42 | 80 | 99 => vec![vec![b'<', b'v', b'A'], vec![b'v', b'<', b'A']], // 30 51 62 84 95
        21 | 78 | 135 => vec![vec![b'<', b'<', b'A']],                                 // 31 64 97
        75 | 132 => vec![vec![b'<', b'<', b'^', b'A'], vec![b'^', b'<', b'<', b'A']],  // 34 67
        129 => vec![
            vec![b'<', b'<', b'^', b'^', b'A'],
            vec![b'^', b'^', b'<', b'<', b'A'],
        ], // 37
        4 => vec![vec![b'>', b'v', b'v', b'A']],                                       // 40
        58 | 115 => vec![vec![b'>', b'>', b'v', b'A'], vec![b'v', b'>', b'>', b'A']],  // 43 76
        310 => vec![vec![b'>', b'>', b'v', b'v', b'A']],                               // 4A
        5 | 312 | 25 | 44 | 63 => vec![vec![b'v', b'v', b'A']], // 50 6A 71 82 93
        311 | 43 | 62 => vec![vec![b'>', b'v', b'v', b'A'], vec![b'v', b'v', b'>', b'A']], // 5A 72 83
        6 | 26 | 45 => vec![vec![b'<', b'v', b'v', b'A'], vec![b'v', b'v', b'<', b'A']], // 60 81 92
        24 | 81 => vec![vec![b'<', b'<', b'v', b'A'], vec![b'v', b'<', b'<', b'A']],     // 61 94
        7 => vec![vec![b'>', b'v', b'v', b'v', b'A']],                                   // 70
        61 => vec![
            vec![b'>', b'>', b'v', b'v', b'A'],
            vec![b'v', b'v', b'>', b'>', b'A'],
        ], // 73
        313 => vec![vec![b'>', b'>', b'v', b'v', b'v', b'A']],                           // 7A
        8 | 315 => vec![vec![b'v', b'v', b'v', b'A']],                                   // 80 9A
        314 => vec![
            vec![b'>', b'v', b'v', b'v', b'A'],
            vec![b'v', b'v', b'v', b'>', b'A'],
        ], // 8A
        9 => vec![
            vec![b'<', b'v', b'v', b'v', b'A'],
            vec![b'v', b'v', b'v', b'<', b'A'],
        ], // 90
        27 => vec![
            vec![b'<', b'<', b'v', b'v', b'A'],
            vec![b'v', b'v', b'<', b'<', b'A'],
        ], // 91
        35 => vec![vec![b'^', b'<', b'<', b'A']],                                        // A1
        89 => vec![vec![b'^', b'^', b'<', b'<', b'A']],                                  // A4
        143 => vec![vec![b'^', b'^', b'^', b'<', b'<', b'A']],                           // A7
        161 => vec![
            vec![b'<', b'^', b'^', b'^', b'A'],
            vec![b'^', b'^', b'^', b'<', b'A'],
        ], // A8
        _ => vec![],
    }
}

fn get_direction_keypad_paths(from: u8, to: u8) -> Vec<Vec<u8>> {
    match from - 60 + (to - 60) * 3 {
        0 | 8 | 20 | 136 | 232 => vec![vec![b'A']], // << >> AA ^^ vv
        6 => vec![vec![b'>', b'>', b'A']],          // <>
        15 => vec![vec![b'>', b'>', b'^', b'A']],   // <A
        102 => vec![vec![b'>', b'^', b'A']],        // <^
        174 | 49 | 64 => vec![vec![b'>', b'A']],    // <v ^A v>
        2 => vec![vec![b'<', b'<', b'A']],          // ><
        17 | 160 => vec![vec![b'^', b'A']],         // >A v^
        104 => vec![vec![b'<', b'^', b'A'], vec![b'^', b'<', b'A']], // >^
        176 | 107 | 58 => vec![vec![b'<', b'A']],   // >v A^ v<
        5 => vec![vec![b'v', b'<', b'<', b'A']],    // A<
        11 | 208 => vec![vec![b'v', b'A']],         // A> ^v
        179 => vec![vec![b'<', b'v', b'A'], vec![b'v', b'<', b'A']], // Av
        34 => vec![vec![b'v', b'<', b'A']],         // ^<
        40 => vec![vec![b'>', b'v', b'A'], vec![b'v', b'>', b'A']], // ^>
        73 => vec![vec![b'>', b'^', b'A'], vec![b'^', b'>', b'A']], // vA
        _ => vec![],
    }
}

unsafe fn recursion(
    depth: u8,
    max_depth: u8,
    from: u8,
    to: u8,
    cache: &mut FxHashMap<(u8, u8, u8), u64>,
) -> u64 {
    if let Some(&cached) = cache.get(&(depth, from, to)) {
        return cached;
    }

    let min_buttons_to_press = get_direction_keypad_paths(from, to)
        .iter()
        .map(|path| {
            if depth == max_depth {
                return path.len() as u64;
            }

            let mut buttons_to_press = 0;
            let mut from_button = b'A';

            for &to_button in path {
                buttons_to_press += recursion(depth + 1, max_depth, from_button, to_button, cache);
                from_button = to_button;
            }

            buttons_to_press
        })
        .min()
        .unwrap_unchecked();

    cache.insert((depth, from, to), min_buttons_to_press);

    min_buttons_to_press
}

unsafe fn parse_num(n: &[u8]) -> u64 {
    (*n.get_unchecked(0) as u64) * 100
        + (*n.get_unchecked(1) as u64) * 10
        + (*n.get_unchecked(2) as u64)
        - 5328
}

unsafe fn solve(input: &str, max_depth: u8, cache_capacity: usize) -> u64 {
    let mut solution = 0;
    let mut cache: FxHashMap<(u8, u8, u8), u64> =
        FxHashMap::with_capacity_and_hasher(cache_capacity, FxBuildHasher::default());

    for code in input.lines() {
        let mut pressed = 0;
        let mut from_numpad_button = b'A';

        for &to_numpad_button in code.as_bytes() {
            let min_buttons_to_press =
                get_numeric_keypad_paths(from_numpad_button, to_numpad_button)
                    .iter()
                    .map(|path| {
                        let mut buttons_to_press = 0;
                        let mut from_button = b'A';

                        for &to_button in path {
                            buttons_to_press +=
                                recursion(1, max_depth, from_button, to_button, &mut cache);
                            from_button = to_button;
                        }

                        buttons_to_press
                    })
                    .min()
                    .unwrap_unchecked();

            pressed += min_buttons_to_press;
            from_numpad_button = to_numpad_button;
        }

        solution += pressed * parse_num(code.as_bytes());

        cache.clear();
    }

    solution
}

pub fn part1(input: &str) -> u64 {
    unsafe { solve(input, 2, 40) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { solve(input, 25, 500) }
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
