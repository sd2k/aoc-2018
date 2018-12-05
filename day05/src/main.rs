use env_logger;

use day05;

fn main() {
    env_logger::init();
    let input = include_str!("../../input/2018/day5.txt");
    println!("Part 1: {}", day05::part1(&input.trim()));
    println!("Part 2: {}", day05::part2(&input.trim()));
}
