use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solution::day10;
use solution::day11;
use solution::day9;

pub fn day9(c: &mut Criterion) {
    let s = read_to_string("./inputs/9.txt").unwrap();
    let s = s.as_str();

    c.bench_function("bench_day9_part1", |b| b.iter(|| day9::part1(black_box(s))));
    c.bench_function("bench_day9_part2", |b| b.iter(|| day9::part2(black_box(s))));
}

pub fn day10(c: &mut Criterion) {
    let s = read_to_string("./inputs/10.txt").unwrap();
    let s = s.as_str();

    c.bench_function("bench_day10_part1", |b| {
        b.iter(|| day10::part1(black_box(s)))
    });
    c.bench_function("bench_day10_part2", |b| {
        b.iter(|| day10::part2(black_box(s)))
    });
}

pub fn day11(c: &mut Criterion) {
    let s = read_to_string("./inputs/11.txt").unwrap();
    let s = s.as_str();

    c.bench_function("bench_day11_part1", |b| {
        b.iter(|| day11::part1(black_box(s)))
    });
    c.bench_function("bench_day11_part2", |b| {
        b.iter(|| day11::part2(black_box(s)))
    });
}

criterion_group!(benches, day9, day10, day11);
criterion_main!(benches);
