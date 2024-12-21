use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;

macro_rules! get_day_input {
    ($day_file:expr) => {
        include_str!(concat!("../inputs/", $day_file))
    };
}

macro_rules! benches_day {
    ($day_name:expr, $day_file:expr) => {
        paste! {
            use solution::[<day $day_name>];

            pub fn [<bench_day $day_name>](c: &mut Criterion) {
                let input = get_day_input!($day_file);
                c.bench_function(&format!("bench_day{}_part1", $day_name), |b| b.iter(|| [<day $day_name>]::part1(black_box(input))));
                c.bench_function(&format!("bench_day{}_part2", $day_name), |b| b.iter(|| [<day $day_name>]::part2(black_box(input))));
            }
        }
    };
}

macro_rules! benches {
    ($($day_name:expr, $day_file:expr),*) => {
        paste! {
            $(
                benches_day!($day_name, $day_file);
            )*

            criterion_group!(benches, $([<bench_day $day_name>]),*);
            criterion_main!(benches);
        }
    };
}

#[rustfmt::skip]
benches!(
    9, "9.txt",
    10, "10.txt",
    11, "11.txt",
    12, "12.txt",
    13, "13.txt", "13_simd", "13.txt",
    14, "14.txt",
    15, "15.txt",
    16, "16.txt",
    17, "17.txt",
    18, "18.txt",
    19, "19.txt",
    20, "20.txt",
    21, "21.txt"
);
