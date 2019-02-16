use std::time;

use day06;

fn main() {
    env_logger::Builder::from_default_env()
        .default_format_timestamp_nanos(true)
        .init();
    let input: Vec<&str> = include_str!("../../input/2018/day6.txt").lines().collect();
    let start = time::SystemTime::now();
    println!("Part 1: {}", day06::part1(&input));
    println!("Part 2: {}", day06::part2(&input, 10000));
    println!(
        "Ran in {:?}",
        time::SystemTime::now().duration_since(start).unwrap()
    );
}
