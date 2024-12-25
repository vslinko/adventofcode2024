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

fn key2(prefix: usize, index: usize) -> usize {
    prefix << 16 | (index / 10 + 48) << 8 | index % 10 + 48
}

fn key3(a: u8, b: u8, c: u8) -> usize {
    (a as usize) << 16 | (b as usize) << 8 | c as usize
}

fn key_to_str(str: &mut String, key: usize) {
    str.push((key >> 16) as u8 as char);
    str.push((key >> 8) as u8 as char);
    str.push(key as u8 as char);
}

pub fn part1(input: &str) -> usize {
    unsafe { inner1(input) }
}

unsafe fn inner1(input: &str) -> usize {
    let input = input.as_bytes();

    let mut x = [0; X_Y_SIZE];
    let mut y = [0; X_Y_SIZE];
    let mut wires = Vec::with_capacity(CONTEXT_SIZE);

    (0..X_Y_SIZE).for_each(|i| {
        *x.get_unchecked_mut(i) =
            (*input.get_unchecked(i * INITIAL_VALUE_LINE_LENGTH + INPUT_X_POS) - b'0') as usize;
        wires.push(key2(X, i));
    });

    (0..X_Y_SIZE).for_each(|i| {
        *y.get_unchecked_mut(i) =
            (*input.get_unchecked(i * INITIAL_VALUE_LINE_LENGTH + INPUT_Y_POS) - b'0') as usize;
        wires.push(key2(Y, i));
    });

    let mut gates = FxHashMap::with_capacity_and_hasher(GATES_COUNT, FxBuildHasher::default());
    let mut edges = FxHashMap::with_capacity_and_hasher(266, FxBuildHasher::default());

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

        wires.push(to);
        gates.insert(to, Gate::new(a, op, b));
        edges.entry(a).or_insert_with(Vec::new).push(to);
        edges.entry(b).or_insert_with(Vec::new).push(to);
    });

    let mut sorted_wires = Vec::with_capacity(CONTEXT_SIZE);
    let mut in_degree = FxHashMap::with_capacity_and_hasher(GATES_COUNT, FxBuildHasher::default());
    let mut stack = Vec::with_capacity(90);

    wires.iter().for_each(|&wire| {
        if let Some(wire_edges) = edges.get(&wire) {
            wire_edges.iter().for_each(|&to| {
                in_degree.entry(to).and_modify(|d| *d += 1).or_insert(1);
            });
        }
    });

    wires.iter().for_each(|&wire| {
        let deg = match in_degree.get(&wire) {
            Some(deg) => *deg,
            None => 0,
        };

        if deg == 0 {
            stack.push(wire);
        }
    });

    while let Some(wire) = stack.pop() {
        sorted_wires.push(wire);

        if let Some(wire_edges) = edges.get(&wire) {
            wire_edges.iter().for_each(|to| {
                let wire_in_degree = in_degree.get_mut(to).unwrap_unchecked();
                *wire_in_degree -= 1;
                if *wire_in_degree == 0 {
                    stack.push(*to);
                }
            });
        }
    }

    let mut outputs = FxHashMap::with_capacity_and_hasher(CONTEXT_SIZE, FxBuildHasher::default());

    macro_rules! get_input_value {
        ($source:expr, $wire:expr) => {{
            let b = ((($wire >> 8) & 0xFF) - 48) * 10;
            let c = ($wire & 0xFF) - 48;
            *$source.get_unchecked(b + c)
        }};
    }

    sorted_wires
        .iter()
        .for_each(|&wire| match (wire >> 16) as u8 {
            b'x' => {
                outputs.insert(wire, get_input_value!(x, wire));
            }
            b'y' => {
                outputs.insert(wire, get_input_value!(y, wire));
            }
            _ => {
                let gate = gates.get(&wire).unwrap_unchecked();
                let a = *outputs.get(&gate.a).unwrap_unchecked();
                let b = *outputs.get(&gate.b).unwrap_unchecked();

                let output = match gate.op {
                    GateOperaion::And => a & b,
                    GateOperaion::Or => a | b,
                    GateOperaion::Xor => a ^ b,
                };

                outputs.insert(wire, output);
            }
        });

    (0..Z_SIZE).rev().fold(0, |acc, i| {
        acc * 2 + outputs.get(&key2(Z, i)).unwrap_unchecked()
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
