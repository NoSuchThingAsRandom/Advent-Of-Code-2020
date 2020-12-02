use advent_of_code_2020::{days::day01::bench, read_vec};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut data = read_vec(String::from("../Inputs/input1.txt")).unwrap();
    let mut x = data.clone();
    c.bench_function("Day 1 Part 2", |b| b.iter(|| bench(&mut x)));
    /*    c.bench_function("fib 20", |b| b.iter(|| test(black_box(20))));
    c.bench_function("fib 20", |b| b.iter(|| (black_box(20))));*/
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
