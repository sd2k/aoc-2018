use criterion::{criterion_group, criterion_main, Criterion};
use day01::{part1, part2_functional, part2_imperative};

fn part1_benchmark(c: &mut Criterion) {
    let input: Vec<&str> = include_str!("../../input/2018/day1.txt").lines().collect();
    c.bench_function("part1", move |b| b.iter(|| part1(&input)));
}

fn part2_functional_benchmark(c: &mut Criterion) {
    let input: Vec<&str> = include_str!("../../input/2018/day1.txt").lines().collect();
    c.bench_function("part2_functional", move |b| {
        b.iter(|| part2_functional(&input))
    });
}

fn part2_imperative_benchmark(c: &mut Criterion) {
    let input: Vec<&str> = include_str!("../../input/2018/day1.txt").lines().collect();
    c.bench_function("part2_imperative", move |b| {
        b.iter(|| part2_imperative(&input))
    });
}

criterion_group!(
    benches,
    part1_benchmark,
    part2_functional_benchmark,
    part2_imperative_benchmark
);
criterion_main!(benches);
