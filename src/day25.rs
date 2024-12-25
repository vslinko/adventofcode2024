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
    let mut locks = [0; WIDTH * LOCKS_COUNT];
    let mut keys = [0; WIDTH * KEYS_COUNT];
    let mut locks_idx = 0;
    let mut keys_idx = 0;

    (0..SCHEMATICS_COUNT).for_each(|i| {
        let schema = match input.get_unchecked(INPUT_SCHEMA_SIZE * i) {
            b'#' => {
                let schema = &mut locks[locks_idx..];
                locks_idx += WIDTH;
                schema
            }
            _ => {
                let schema = &mut keys[keys_idx..];
                keys_idx += WIDTH;
                schema
            }
        };

        (0..HEIGHT).for_each(|r| {
            (0..WIDTH).for_each(|c| {
                if *input.get_unchecked(INPUT_SCHEMA_SIZE * i + INPUT_ROW_SIZE * r + c) == b'#' {
                    *schema.get_unchecked_mut(c) += 1;
                }
            });
        });
    });

    let mut result = 0;

    (0..LOCKS_COUNT).for_each(|l| {
        (0..KEYS_COUNT).for_each(|k| {
            macro_rules! return_if_not_match {
                ($i:expr) => {
                    if *locks.get_unchecked(l * WIDTH + $i) + *keys.get_unchecked(k * WIDTH + $i)
                        > HEIGHT
                    {
                        return;
                    }
                };
            }

            return_if_not_match!(0);
            return_if_not_match!(1);
            return_if_not_match!(2);
            return_if_not_match!(3);
            return_if_not_match!(4);

            result += 1;
        });
    });

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
