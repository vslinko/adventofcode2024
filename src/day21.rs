use std::collections::HashMap;

fn get_numeric_keypad_paths(from: &str, to: &str) -> Vec<String> {
    match (from, to) {
        ("A", "7") => vec!["^^^<<A".to_string()],
        ("7", "8") => vec![">A".to_string()],
        ("8", "9") => vec![">A".to_string()],
        ("9", "A") => vec!["vvvA".to_string()],
        ("A", "0") => vec!["<A".to_string()],
        ("0", "2") => vec!["^A".to_string()],
        ("2", "9") => vec!["^^>A".to_string(), ">^^A".to_string()],
        ("A", "9") => vec!["^^^A".to_string()],
        ("9", "8") => vec!["<A".to_string()],
        ("8", "0") => vec!["vvvA".to_string()],
        ("0", "A") => vec![">A".to_string()],
        ("A", "1") => vec!["^<<A".to_string()],
        ("1", "7") => vec!["^^A".to_string()],
        ("7", "9") => vec![">>A".to_string()],
        ("A", "4") => vec!["^^<<A".to_string()],
        ("4", "5") => vec![">A".to_string()],
        ("5", "6") => vec![">A".to_string()],
        ("6", "A") => vec!["vvA".to_string()],
        ("A", "3") => vec!["^A".to_string()],
        ("3", "7") => vec!["^^<<A".to_string(), "<<^^A".to_string()],
        ("9", "6") => vec!["vA".to_string()],
        ("6", "8") => vec!["^<A".to_string(), "<^A".to_string()],
        ("8", "A") => vec!["vvv>A".to_string(), ">vvvA".to_string()],
        ("A", "2") => vec!["^<A".to_string(), "<^A".to_string()],
        ("2", "8") => vec!["^^A".to_string()],
        ("8", "6") => vec![">vA".to_string(), "v>A".to_string()],
        ("3", "4") => vec!["^<<A".to_string(), "<<^A".to_string()],
        ("4", "9") => vec!["^>>A".to_string(), ">>^A".to_string()],
        ("7", "0") => vec![">vvvA".to_string()],
        _ => panic!("Unknown move from {} to {}", from, to),
    }
}

fn get_direction_keypad_paths(from: &str, to: &str) -> Vec<String> {
    match (from, to) {
        (f, t) if f == t => vec!["A".to_string()],
        ("A", "<") => vec!["v<<A".to_string()],
        ("<", "A") => vec![">>^A".to_string()],
        ("A", "v") => vec!["v<A".to_string(), "<vA".to_string()],
        ("v", "<") => vec!["<A".to_string()],
        ("A", ">") => vec!["vA".to_string()],
        (">", "^") => vec!["<^A".to_string(), "^<A".to_string()],
        ("^", "A") => vec![">A".to_string()],
        ("A", "^") => vec!["<A".to_string()],
        (">", "A") => vec!["^A".to_string()],
        ("^", ">") => vec!["v>A".to_string(), ">vA".to_string()],
        ("v", ">") => vec![">A".to_string()],
        ("v", "A") => vec![">^A".to_string(), "^>A".to_string()],
        ("<", "^") => vec![">^A".to_string()],
        ("^", "<") => vec!["v<A".to_string()],
        (">", "v") => vec!["<A".to_string()],
        ("<", "v") => vec![">A".to_string()],
        _ => panic!("Unknown move from {} to {}", from, to),
    }
}

fn recursion(
    depth: usize,
    max_depth: usize,
    from: &str,
    to: &str,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    if depth == max_depth {
        return 1;
    }

    let cache_key = format!("{},{},{}", depth, from, to);
    if let Some(&cached) = cache.get(&cache_key) {
        return cached;
    }

    let mut min_buttons_to_press = i64::MAX;

    for path in get_direction_keypad_paths(from, to) {
        let mut buttons_to_press = 0;
        let mut from_button = "A".to_string();

        for to_button in path.chars() {
            buttons_to_press += recursion(
                depth + 1,
                max_depth,
                &from_button,
                &to_button.to_string(),
                cache,
            );

            from_button = to_button.to_string();
        }

        min_buttons_to_press = min_buttons_to_press.min(buttons_to_press);
    }

    cache.insert(cache_key, min_buttons_to_press);

    min_buttons_to_press
}

fn solve(input: &str, max_depth: usize) -> i64 {
    let mut solution = 0;

    for code in input.lines() {
        let mut cache: HashMap<String, i64> = HashMap::new();

        let mut pressed = 0;
        let mut from_numpad_button = "A".to_string();

        for to_numpad_button in code.chars() {
            let paths =
                get_numeric_keypad_paths(&from_numpad_button, &to_numpad_button.to_string());

            let mut min_buttons_to_press = i64::MAX;

            for path in paths {
                let mut buttons_to_press = 0;
                let mut from_button = "A".to_string();

                for to_button in path.chars() {
                    buttons_to_press += recursion(
                        0,
                        max_depth,
                        &from_button,
                        &to_button.to_string(),
                        &mut cache,
                    );
                    from_button = to_button.to_string();
                }

                min_buttons_to_press = min_buttons_to_press.min(buttons_to_press);
            }

            pressed += min_buttons_to_press;
            from_numpad_button = to_numpad_button.to_string();
        }

        let code_number = code[..code.len() - 1].parse::<i64>().unwrap();
        solution += pressed * code_number;
    }

    solution
}

pub fn part1(input: &str) -> i64 {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> i64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> i64 {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> i64 {
    solve(input, 25)
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
