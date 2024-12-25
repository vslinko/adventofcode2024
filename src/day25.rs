const LOCKS_COUNT: usize = 250;
const KEYS_COUNT: usize = 250;
const SCHEMATICS_COUNT: usize = LOCKS_COUNT + KEYS_COUNT;
const WIDTH: usize = 5;
const HEIGHT: usize = 7;
const INPUT_ROW_SIZE: usize = WIDTH + 1;
const INPUT_SCHEMA_SIZE: usize = INPUT_ROW_SIZE * HEIGHT + 1;

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut locks = Vec::with_capacity(LOCKS_COUNT);
    let mut keys = Vec::with_capacity(KEYS_COUNT);

    (0..SCHEMATICS_COUNT).for_each(|i| {
        let mut schema = [0; WIDTH];

        let is_lock = *input.get_unchecked(INPUT_SCHEMA_SIZE * i) == b'#';

        (0..HEIGHT).for_each(|r| {
            (0..WIDTH).for_each(|c| {
                if *input.get_unchecked(INPUT_SCHEMA_SIZE * i + INPUT_ROW_SIZE * r + c) == b'#' {
                    schema[c] += 1;
                }
            });
        });

        if is_lock {
            locks.push(schema);
        } else {
            keys.push(schema);
        }
    });

    let mut result = 0;

    for lock in locks.iter() {
        'qwe: for key in keys.iter() {
            for i in 0..WIDTH {
                if lock[i] + key[i] > 7 {
                    continue 'qwe;
                }
            }
            result += 1;
        }
    }

    result
}

pub fn part2(_input: &str) -> u8 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day25_part1() {
        let prod_input = read_to_string("./inputs/25.txt").unwrap();
        let prod_output = read_to_string("./outputs/25p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }
}
