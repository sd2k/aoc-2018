use day03;

fn main() {
    let input: Vec<&str> = include_str!("../../input/2018/day3.txt").lines().collect();
    println!("Part 1: {}", day03::part1(&input));
    println!("Part 2: {}", day03::part2(&input));
}
