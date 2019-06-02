use criterion::{criterion_group, criterion_main, Criterion};
use day11::part1;

fn part1_benchmark(c: &mut Criterion) {
    c.bench_function("part1", move |b| b.iter(|| part1(9306, 300, 300)));
}

// fn part2_benchmark(c: &mut Criterion) {
//     let input = include_str!("../../input/2018/day8.txt");
//     c.bench_function("part2", move |b| b.iter(|| part2(&input)));
// }

criterion_group!(
    benches,
    part1_benchmark,
    //part2_benchmark
);
criterion_main!(benches);
