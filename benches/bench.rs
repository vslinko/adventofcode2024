use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solution::day10::{part1, part2};

pub fn day10(c: &mut Criterion) {
    let s = read_to_string("./inputs/10.txt").unwrap();
    let s = s.as_str();

    c.bench_function("day10 part1", |b| b.iter(|| part1(black_box(s))));
    c.bench_function("day10 part2", |b| b.iter(|| part2(black_box(s))));
}

criterion_group!(benches, day10);
criterion_main!(benches);
