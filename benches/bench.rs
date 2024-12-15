use std::fs::read_to_string;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solution::day10;
use solution::day11;
use solution::day12;
use solution::day13;
use solution::day14;
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

pub fn day12(c: &mut Criterion) {
    let s = read_to_string("./inputs/12.txt").unwrap();
    let s = s.as_str();

    c.bench_function("bench_day12_part1", |b| {
        b.iter(|| day12::part1(black_box(s)))
    });
    c.bench_function("bench_day12_part2", |b| {
        b.iter(|| day12::part2(black_box(s)))
    });
}

pub fn day13(c: &mut Criterion) {
    let s = read_to_string("./inputs/13.txt").unwrap();
    let s = s.as_str();

    c.bench_function("bench_day13_part1", |b| {
        b.iter(|| day13::part1(black_box(s)))
    });
    c.bench_function("bench_day13_part2", |b| {
        b.iter(|| day13::part2(black_box(s)))
    });
}

pub fn day14(c: &mut Criterion) {
    let s = read_to_string("./inputs/14.txt").unwrap();
    let s = s.as_str();

    c.bench_function("bench_day14_part1", |b| {
        b.iter(|| day14::part1(black_box(s)))
    });
    c.bench_function("bench_day14_part2", |b| {
        b.iter(|| day14::part2(black_box(s)))
    });
}

criterion_group!(benches, day9, day10, day11, day12, day13, day14);
criterion_main!(benches);
