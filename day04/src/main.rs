use env_logger;

use day04;

fn main() {
    env_logger::init();
    let input: Vec<&str> = include_str!("../../input/2018/day4.txt").lines().collect();
    println!("Part 1: {}", day04::part1(&input));
    println!("Part 2: {}", day04::part2(&input));
}
