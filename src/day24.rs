use rustc_hash::{FxBuildHasher, FxHashMap};

const X_Y_SIZE: usize = 45;
const Z_SIZE: usize = 46;
const GATES_COUNT: usize = 222;
const INITIAL_VALUE_LINE_LENGTH: usize = 7;
const INPUT_X_POS: usize = 5;
const INPUT_Y_POS: usize = X_Y_SIZE * INITIAL_VALUE_LINE_LENGTH + INPUT_X_POS;
const INPUT_GATES_POS: usize = (X_Y_SIZE + X_Y_SIZE) * INITIAL_VALUE_LINE_LENGTH + 1;
const X: usize = b'x' as usize;
const Y: usize = b'y' as usize;
const Z: usize = b'z' as usize;
const CONTEXT_SIZE: usize = GATES_COUNT + X_Y_SIZE * 2;

struct InitialValue {
    val: usize,
}

impl InitialValue {
    fn new(val: usize) -> Self {
        InitialValue { val }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum GateOperaion {
    And,
    Or,
    Xor,
}

#[derive(Clone)]
struct Gate {
    a: usize,
    op: GateOperaion,
    b: usize,
}

impl Gate {
    fn new(a: usize, op: GateOperaion, b: usize) -> Self {
        Gate { a, op, b }
    }
}

enum ReadableValue {
    Initial(InitialValue),
    Gate(Gate),
}

impl ReadableValue {
    unsafe fn read(&self, context: &FxHashMap<usize, ReadableValue>) -> usize {
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

fn key2(prefix: usize, index: usize) -> usize {
    prefix * 65536 + ((index / 10) + 48) * 256 + ((index % 10) + 48)
}

fn key3(a: u8, b: u8, c: u8) -> usize {
    a as usize * 65536 + b as usize * 256 + c as usize
}

fn key_to_str(str: &mut String, key: usize) {
    str.push(((key >> 16) & 0xFF) as u8 as char);
    str.push(((key >> 8) & 0xFF) as u8 as char);
    str.push((key & 0xFF) as u8 as char);
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut context = FxHashMap::with_capacity_and_hasher(CONTEXT_SIZE, FxBuildHasher::default());

    (0..X_Y_SIZE).for_each(|i| {
        let index = i * INITIAL_VALUE_LINE_LENGTH + INPUT_X_POS;
        let val = (*input.get_unchecked(index) - b'0') as usize;
        context.insert(key2(X, i), ReadableValue::Initial(InitialValue::new(val)));
    });

    (0..X_Y_SIZE).for_each(|i| {
        let index = i * INITIAL_VALUE_LINE_LENGTH + INPUT_Y_POS;
        let val = (*input.get_unchecked(index) - b'0') as usize;
        context.insert(key2(Y, i), ReadableValue::Initial(InitialValue::new(val)));
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

    (0..Z_SIZE).rev().fold(0, |acc, i| {
        acc * 2 + context.get(&key2(Z, i)).unwrap_unchecked().read(&context)
    })
}

unsafe fn swap(context: &mut FxHashMap<usize, Gate>, swaps: &mut Vec<usize>, a: usize, b: usize) {
    let tmp = context.get(&a).unwrap_unchecked().clone();
    context.insert(a, context.get(&b).unwrap_unchecked().clone());
    context.insert(b, tmp.clone());
    swaps.push(a);
    swaps.push(b);
}

fn gate_output(context: &FxHashMap<usize, Gate>, a: usize, op: GateOperaion, b: usize) -> usize {
    context
        .iter()
        .find(|(_, gate)| {
            gate.op == op && ((gate.a == a && gate.b == b) || (gate.a == b && gate.b == a))
        })
        .map(|(output, _)| *output)
        .unwrap_or(usize::MAX)
}

pub fn part2(input: &str) -> String {
    unsafe { inner2(input) }
}

unsafe fn inner2(input: &str) -> String {
    let input = input.as_bytes();
    let mut context: FxHashMap<usize, Gate> =
        FxHashMap::with_capacity_and_hasher(GATES_COUNT, FxBuildHasher::default());

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

        context.insert(to, Gate::new(a, op, b));
    });

    let mut swaps = Vec::with_capacity(8);
    let mut prev_output = gate_output(&context, key2(X, 0), GateOperaion::And, key2(Y, 0));

    (1..Z_SIZE - 1).for_each(|i| {
        let x = key2(X, i);
        let y = key2(Y, i);
        let z = key2(Z, i);

        loop {
            let xor_gate_output = gate_output(&context, x, GateOperaion::Xor, y);
            let z_gate = context.get(&z).unwrap_unchecked();
            let z_gate_input_a = z_gate.a;
            let z_gate_input_b = z_gate.b;

            if z_gate_input_a == prev_output && z_gate_input_b != xor_gate_output {
                swap(&mut context, &mut swaps, z_gate_input_b, xor_gate_output);
                continue;
            }

            if z_gate_input_b == prev_output && z_gate_input_a != xor_gate_output {
                swap(&mut context, &mut swaps, z_gate_input_a, xor_gate_output);
                continue;
            }

            let z_gate_output =
                gate_output(&context, xor_gate_output, GateOperaion::Xor, prev_output);

            if z_gate_output != z {
                swap(&mut context, &mut swaps, z_gate_output, z);
                continue;
            }

            let and_gate_output1 = gate_output(&context, x, GateOperaion::And, y);
            let and_gate_output2 =
                gate_output(&context, xor_gate_output, GateOperaion::And, prev_output);

            prev_output = gate_output(
                &context,
                and_gate_output1,
                GateOperaion::Or,
                and_gate_output2,
            );

            break;
        }
    });

    swaps.sort_unstable();

    swaps.iter().skip(1).fold(
        {
            let mut str = String::with_capacity(4 * 8 - 1);
            key_to_str(&mut str, *swaps.get_unchecked(0));
            str
        },
        |mut str, key| {
            str.push(',');
            key_to_str(&mut str, *key);
            str
        },
    )
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
