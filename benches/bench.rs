use criterion::{black_box, criterion_group, criterion_main, Criterion};
use paste::paste;

macro_rules! get_day_input {
    ($day_name:expr) => {
        include_str!(concat!("../inputs/", $day_name, ".txt"))
    };
}

macro_rules! benches_day {
    ($day_name:expr) => {
        paste! {
            use solution::[<day $day_name>];

            pub fn [<bench_day $day_name>](c: &mut Criterion) {
                let input = get_day_input!($day_name);
                c.bench_function(&format!("bench_day{}_part1", $day_name), |b| b.iter(|| [<day $day_name>]::part1(black_box(input))));
                c.bench_function(&format!("bench_day{}_part2", $day_name), |b| b.iter(|| [<day $day_name>]::part2(black_box(input))));
            }
        }
    };
}

macro_rules! benches {
    ($($day_name:expr),*) => {
        paste! {
            $(
                benches_day!($day_name);
            )*

            criterion_group!(benches, $([<bench_day $day_name>]),*);
            criterion_main!(benches);
        }
    };
}

benches!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25);
