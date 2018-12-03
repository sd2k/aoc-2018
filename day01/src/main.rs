use day01;

fn main() {
    let input: Vec<&str> = include_str!("../../input/2018/day1.txt").lines().collect();
    println!("Part 1: {}", day01::part1(&input));
    println!("Part 2: {}", day01::part2_functional(&input));
}
