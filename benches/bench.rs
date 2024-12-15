use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;

macro_rules! get_day_input {
    ($day_num:literal) => {
        include_str!(concat!("../inputs/", $day_num, ".txt"))
    };
}

macro_rules! benches_day {
    ($day_num:literal) => {
        paste! {
            use solution::[<day $day_num>];

            pub fn [<bench_day $day_num>](c: &mut Criterion) {
                let input = get_day_input!($day_num);
                c.bench_function(&format!("bench_day{}_part1", $day_num), |b| b.iter(|| [<day $day_num>]::part1(black_box(input))));
                c.bench_function(&format!("bench_day{}_part2", $day_num), |b| b.iter(|| [<day $day_num>]::part2(black_box(input))));
            }
        }
    };
}

macro_rules! benches {
    ($($day_num:literal),*) => {
        paste! {
            $(
                benches_day!($day_num);
            )*

            criterion_group!(benches, $([<bench_day $day_num>]),*);
            criterion_main!(benches);
        }
    };
}

benches!(9, 10, 11, 12, 13, 14);
