use criterion::{criterion_group, criterion_main, Criterion};
use day11::{part1, part2};

fn part1_benchmark(c: &mut Criterion) {
    c.bench_function("part1", move |b| b.iter(|| part1(9306, 300, 300)));
}

fn part2_benchmark(c: &mut Criterion) {
    c.bench_function("part2", move |b| b.iter(|| part2(9306, 300)));
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
criterion_main!(benches);
