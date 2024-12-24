use rustc_hash::{FxBuildHasher, FxHashMap};

const X_Y_SIZE: usize = 45;
const X_Y_SIZE_U32: u32 = X_Y_SIZE as u32;
const Z_SIZE: usize = 46;
const Z_SIZE_U32: u32 = Z_SIZE as u32;
const GATES_COUNT: usize = 222;
const INITIAL_VALUE_LINE_LENGTH: usize = 7;
const INPUT_X_POS: usize = 5;
const INPUT_Y_POS: usize = X_Y_SIZE * INITIAL_VALUE_LINE_LENGTH + INPUT_X_POS;
const INPUT_GATES_POS: usize = (X_Y_SIZE + X_Y_SIZE) * INITIAL_VALUE_LINE_LENGTH + 1;
const X_U32: u32 = b'x' as u32;
const Y_U32: u32 = b'y' as u32;
const Z_U32: u32 = b'z' as u32;
const CONTEXT_SIZE: usize = GATES_COUNT + X_Y_SIZE * 2;

type Key = u32;

struct InitialValue {
    val: usize,
}

impl InitialValue {
    fn new(val: usize) -> Self {
        InitialValue { val }
    }
}

enum GateOperaion {
    And,
    Or,
    Xor,
}

struct Gate {
    a: Key,
    op: GateOperaion,
    b: Key,
}

impl Gate {
    fn new(a: Key, op: GateOperaion, b: Key) -> Self {
        Gate { a, op, b }
    }
}

enum ReadableValue {
    Initial(InitialValue),
    Gate(Gate),
}

type Context = FxHashMap<Key, ReadableValue>;

impl ReadableValue {
    unsafe fn read(&self, context: &Context) -> usize {
        match self {
            ReadableValue::Initial(val) => val.val,
            ReadableValue::Gate(gate) => {
                let a = context.get(&gate.a).unwrap_unchecked().read(context);
                let b = context.get(&gate.b).unwrap_unchecked().read(context);

                match gate.op {
                    GateOperaion::And => a & b,
                    GateOperaion::Or => a | b,
                    GateOperaion::Xor => a ^ b,
                }
            }
        }
    }
}

fn key2(prefix: u32, index: u32) -> u32 {
    if index < 10 {
        prefix as u32 + 12288 + (index + 48) * 65536
    } else {
        prefix as u32 + ((index / 10) + 48) * 256 + ((index % 10) + 48) * 65536
    }
}

fn key3(a: u8, b: u8, c: u8) -> u32 {
    a as u32 + b as u32 * 256 + c as u32 * 65536
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut context: Context =
        FxHashMap::with_capacity_and_hasher(CONTEXT_SIZE, FxBuildHasher::default());

    (0..X_Y_SIZE_U32).for_each(|i| {
        let index = (i as usize) * INITIAL_VALUE_LINE_LENGTH + INPUT_X_POS;
        let val = (*input.get_unchecked(index) - b'0') as usize;
        context.insert(
            key2(X_U32, i),
            ReadableValue::Initial(InitialValue::new(val)),
        );
    });

    (0..X_Y_SIZE_U32).for_each(|i| {
        let index = (i as usize) * INITIAL_VALUE_LINE_LENGTH + INPUT_Y_POS;
        let val = (*input.get_unchecked(index) - b'0') as usize;
        context.insert(
            key2(Y_U32, i),
            ReadableValue::Initial(InitialValue::new(val)),
        );
    });

    let mut i = INPUT_GATES_POS;
    (0..GATES_COUNT).for_each(|_| {
        let a = key3(
            *input.get_unchecked(i),
            *input.get_unchecked(i + 1),
            *input.get_unchecked(i + 2),
        );

        let op = match input.get_unchecked(i + 4) {
            b'X' => {
                i += 8;
                GateOperaion::Xor
            }
            b'A' => {
                i += 8;
                GateOperaion::And
            }
            b'O' => {
                i += 7;
                GateOperaion::Or
            }
            _ => unreachable!(),
        };

        let b = key3(
            *input.get_unchecked(i),
            *input.get_unchecked(i + 1),
            *input.get_unchecked(i + 2),
        );

        let to = key3(
            *input.get_unchecked(i + 7),
            *input.get_unchecked(i + 8),
            *input.get_unchecked(i + 9),
        );

        i += 11;

        context.insert(to, ReadableValue::Gate(Gate::new(a, op, b)));
    });

    (0..Z_SIZE_U32).rev().fold(0, |acc, i| {
        acc * 2
            + context
                .get(&key2(Z_U32, i))
                .unwrap_unchecked()
                .read(&context)
    })
}

pub fn part2(_input: &str) -> String {
    "123".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_day24_part1() {
        let prod_input = read_to_string("./inputs/24.txt").unwrap();
        let prod_output = read_to_string("./outputs/24p1.txt").unwrap();
        assert_eq!(part1(&prod_input).to_string(), prod_output);
    }

    #[test]
    fn test_day24_part2() {
        let prod_input = read_to_string("./inputs/24.txt").unwrap();
        let prod_output = read_to_string("./outputs/24p2.txt").unwrap();
        assert_eq!(part2(&prod_input).to_string(), prod_output);
    }
}
