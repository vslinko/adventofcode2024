use std::ops::AddAssign;
use std::simd::prelude::*;

const LOCKS_COUNT: usize = 250;
const KEYS_COUNT: usize = 250;
const SCHEMATICS_COUNT: usize = LOCKS_COUNT + KEYS_COUNT;
const WIDTH: usize = 5;
const HEIGHT: usize = 7;
const HEIGHT_I8: i8 = HEIGHT as i8;
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
    let hash = u8x8::splat(b'#');
    let limit = i8x8::splat(HEIGHT_I8);

    (0..SCHEMATICS_COUNT).for_each(|i| {
        macro_rules! read_row {
            ($schema:expr, $row:expr) => {
                let start = INPUT_SCHEMA_SIZE * i + INPUT_ROW_SIZE * $row;

                $schema.add_assign(
                    u8x8::load_or_default(input.get_unchecked(start..start + WIDTH))
                        .simd_ne(hash)
                        .to_int(),
                );
            };
        }

        macro_rules! read_six_rows {
            ($schema:expr, $shift:expr) => {
                read_row!($schema, 0 + $shift);
                read_row!($schema, 1 + $shift);
                read_row!($schema, 2 + $shift);
                read_row!($schema, 3 + $shift);
                read_row!($schema, 4 + $shift);
                read_row!($schema, 5 + $shift);
            };
        }

        match input.get_unchecked(INPUT_SCHEMA_SIZE * i) {
            b'#' => {
                read_six_rows!(&mut locks[locks_idx], 1);
                locks_idx += 1;
            }
            _ => {
                read_six_rows!(&mut keys[keys_idx], 0);
                keys_idx += 1;
            }
        };
    });

    (0..LOCKS_COUNT).fold(0, |acc, l| {
        acc + (0..KEYS_COUNT).fold(0, |acc, k| {
            if (locks.get_unchecked(l) + keys.get_unchecked(k))
                .simd_gt(limit)
                .any()
            {
                return acc;
            }

            acc + 1
        })
    })
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
