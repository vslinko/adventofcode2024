use rustc_hash::{FxBuildHasher, FxHashMap};

fn get_numeric_keypad_paths(from: u8, to: u8) -> Vec<Vec<u8>> {
    match (from, to) {
        (b'A', b'7') => vec![vec![b'^', b'^', b'^', b'<', b'<', b'A']],
        (b'7', b'8') => vec![vec![b'>', b'A']],
        (b'8', b'9') => vec![vec![b'>', b'A']],
        (b'9', b'A') => vec![vec![b'v', b'v', b'v', b'A']],
        (b'A', b'0') => vec![vec![b'<', b'A']],
        (b'0', b'2') => vec![vec![b'^', b'A']],
        (b'2', b'9') => vec![vec![b'^', b'^', b'>', b'A'], vec![b'>', b'^', b'^', b'A']],
        (b'A', b'9') => vec![vec![b'^', b'^', b'^', b'A']],
        (b'9', b'8') => vec![vec![b'<', b'A']],
        (b'8', b'0') => vec![vec![b'v', b'v', b'v', b'A']],
        (b'0', b'A') => vec![vec![b'>', b'A']],
        (b'A', b'1') => vec![vec![b'^', b'<', b'<', b'A']],
        (b'1', b'7') => vec![vec![b'^', b'^', b'A']],
        (b'7', b'9') => vec![vec![b'>', b'>', b'A']],
        (b'A', b'4') => vec![vec![b'^', b'^', b'<', b'<', b'A']],
        (b'4', b'5') => vec![vec![b'>', b'A']],
        (b'5', b'6') => vec![vec![b'>', b'A']],
        (b'6', b'A') => vec![vec![b'v', b'v', b'A']],
        (b'A', b'3') => vec![vec![b'^', b'A']],
        (b'3', b'7') => vec![
            vec![b'^', b'^', b'<', b'<', b'A'],
            vec![b'<', b'<', b'^', b'^', b'A'],
        ],
        (b'9', b'6') => vec![vec![b'v', b'A']],
        (b'6', b'8') => vec![vec![b'^', b'<', b'A'], vec![b'<', b'^', b'A']],
        (b'8', b'A') => vec![
            vec![b'v', b'v', b'v', b'>', b'A'],
            vec![b'>', b'v', b'v', b'v', b'A'],
        ],
        (b'A', b'2') => vec![vec![b'^', b'<', b'A'], vec![b'<', b'^', b'A']],
        (b'2', b'8') => vec![vec![b'^', b'^', b'A']],
        (b'8', b'6') => vec![vec![b'>', b'v', b'A'], vec![b'v', b'>', b'A']],
        (b'3', b'4') => vec![vec![b'^', b'<', b'<', b'A'], vec![b'<', b'<', b'^', b'A']],
        (b'4', b'9') => vec![vec![b'^', b'>', b'>', b'A'], vec![b'>', b'>', b'^', b'A']],
        (b'7', b'0') => vec![vec![b'>', b'v', b'v', b'v', b'A']],
        _ => panic!("Unknown move from {} to {}", from as char, to as char),
    }
}

fn get_direction_keypad_paths(from: u8, to: u8) -> Vec<Vec<u8>> {
    match (from, to) {
        (f, t) if f == t => vec![vec![b'A']],
        (b'A', b'<') => vec![vec![b'v', b'<', b'<', b'A']],
        (b'<', b'A') => vec![vec![b'>', b'>', b'^', b'A']],
        (b'A', b'v') => vec![vec![b'v', b'<', b'A'], vec![b'<', b'v', b'A']],
        (b'v', b'<') => vec![vec![b'<', b'A']],
        (b'A', b'>') => vec![vec![b'v', b'A']],
        (b'>', b'^') => vec![vec![b'<', b'^', b'A'], vec![b'^', b'<', b'A']],
        (b'^', b'A') => vec![vec![b'>', b'A']],
        (b'A', b'^') => vec![vec![b'<', b'A']],
        (b'>', b'A') => vec![vec![b'^', b'A']],
        (b'^', b'>') => vec![vec![b'v', b'>', b'A'], vec![b'>', b'v', b'A']],
        (b'v', b'>') => vec![vec![b'>', b'A']],
        (b'v', b'A') => vec![vec![b'>', b'^', b'A'], vec![b'^', b'>', b'A']],
        (b'<', b'^') => vec![vec![b'>', b'^', b'A']],
        (b'^', b'<') => vec![vec![b'v', b'<', b'A']],
        (b'>', b'v') => vec![vec![b'<', b'A']],
        (b'<', b'v') => vec![vec![b'>', b'A']],
        _ => panic!("Unknown move from {} to {}", from as char, to as char),
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
