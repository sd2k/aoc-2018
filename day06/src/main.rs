use day06;

fn main() {
    let input: Vec<&str> = include_str!("../../input/2018/day6.txt").lines().collect();
    println!("Part 1: {}", day06::part1(&input));
    println!("Part 2: {}", day06::part2(&input, 10000));
}
