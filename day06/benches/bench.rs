use criterion::{criterion_group, criterion_main, Criterion};
use day06::{part1, part2};

fn part1_benchmark(c: &mut Criterion) {
    let input: Vec<&str> = include_str!("../../input/2018/day6.txt").lines().collect();
    c.bench_function("part1", move |b| b.iter(|| part1(&input)));
}

fn part2_benchmark(c: &mut Criterion) {
    let input: Vec<&str> = include_str!("../../input/2018/day6.txt").lines().collect();
    c.bench_function("part2", move |b| b.iter(|| part2(&input, 10000)));
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
criterion_main!(benches);
