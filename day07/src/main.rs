use day07;

fn main() {
    let input: Vec<&str> = include_str!("../../input/2018/day7.txt").lines().collect();
    println!("Part 1: {}", day07::part1(&input));
    println!("Part 2: {}", day07::part2(&input, 5, 60));
}
