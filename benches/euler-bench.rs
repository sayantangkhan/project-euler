use criterion::{criterion_group, criterion_main, Criterion};
use euler::*;
// use std::env;
// use std::path::Path;

// fn criterion_day_1_1_benchmark(c: &mut Criterion) {
//     let filename = Path::new(env!("CARGO_MANIFEST_DIR"))
//         .join("inputs/day_1_1")
//         .into_os_string()
//         .into_string()
//         .unwrap();
//     c.bench_function("Day 1: Part 1 benchmark", |b| b.iter(|| day_1_1(&filename)));
// }

fn _problem_60_bench(c: &mut Criterion) {
    c.bench_function("Problem 60 bench: Primes <= 10000, n = 5", |b| {
        b.iter(|| problem60(5))
    });
}

// fn problem_201_medium_bench(c: &mut Criterion) {
//     let sorted_numset: Vec<usize> = (1..31).map(|x| x * x).collect();
//     let subset_size = 5;
//     c.bench_function("Problem 201 medium bench", |b| {
//         b.iter(|| problem201(&sorted_numset, subset_size))
//     });
// }

// fn problem_201_medium_hard_bench(c: &mut Criterion) {
//     let sorted_numset: Vec<usize> = (1..101).map(|x| x * x).collect();
//     let subset_size = 10;
//     c.bench_function("Problem 201 medium hard bench", |b| {
//         b.iter(|| problem201(&sorted_numset, subset_size))
//     });
// }

fn problem_202_easy_bench(c: &mut Criterion) {
    c.bench_function("Problem 202 easy bench", |b| b.iter(|| problem202(1000001)));
}

fn _problem_202_hard_bench(c: &mut Criterion) {
    c.bench_function("Problem 202 hard bench", |b| {
        b.iter(|| problem202(12017639147))
    });
}

fn problem_209_bench(c: &mut Criterion) {
    c.bench_function("Problem 209 bench", |b| b.iter(|| problem209()));
}

fn _problem_208_easy_bench(c: &mut Criterion) {
    c.bench_function("Problem 208 easy bench", |b| b.iter(|| problem208(25)));
}

fn _problem_208_hard_bench(c: &mut Criterion) {
    c.bench_function("Problem 208 hard bench", |b| b.iter(|| problem208(70)));
}

criterion_group!(
    benches,
    // problem_201_medium_bench,
    // problem_201_medium_hard_bench
    problem_202_easy_bench,
    // problem_202_hard_bench,
    problem_209_bench,
    _problem_208_easy_bench,
    _problem_208_hard_bench,
);

criterion_main!(benches);
