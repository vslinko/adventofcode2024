use std::{ops::AddAssign, simd::prelude::*};

const LOCKS_COUNT: usize = 250;
const KEYS_COUNT: usize = 250;
const SCHEMATICS_COUNT: usize = LOCKS_COUNT + KEYS_COUNT;
const WIDTH: usize = 5;
const HEIGHT: usize = 7;
const HEIGHT_I8: i8 = 7;
const INPUT_ROW_SIZE: usize = WIDTH + 1;
const INPUT_SCHEMA_SIZE: usize = INPUT_ROW_SIZE * HEIGHT + 1;

pub fn part1(input: &str) -> isize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> isize {
    let input = input.as_bytes();
    let mut locks = [i8x8::splat(HEIGHT_I8); LOCKS_COUNT];
    let mut keys = [i8x8::splat(HEIGHT_I8); KEYS_COUNT];
    let mut locks_idx = 0;
    let mut keys_idx = 0;
    let pattern = mask8x8::from_array([true, true, true, true, true, false, false, false]);
    let hash = u8x8::splat(b'#');
    let limit = i8x8::splat(HEIGHT_I8);

    (0..SCHEMATICS_COUNT).for_each(|i| {
        let schema = match input.get_unchecked(INPUT_SCHEMA_SIZE * i) {
            b'#' => {
                let schema = &mut locks[locks_idx];
                locks_idx += 1;
                schema
            }
            _ => {
                let schema = &mut keys[keys_idx];
                keys_idx += 1;
                schema
            }
        };

        (0..HEIGHT).for_each(|r| {
            let row = u8x8::load_select_or_default(
                input.get_unchecked(INPUT_SCHEMA_SIZE * i + INPUT_ROW_SIZE * r..),
                pattern,
            )
            .simd_ne(hash)
            .to_int();

            schema.add_assign(row);
        });
    });

    let mut result = 0;

    (0..LOCKS_COUNT).for_each(|l| {
        (0..KEYS_COUNT).for_each(|k| {
            if (locks.get_unchecked(l) + keys.get_unchecked(k))
                .simd_gt(limit)
                .any()
            {
                return;
            }

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
