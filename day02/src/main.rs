use day02;

fn main() {
    let input: Vec<&str> = include_str!("../../input/2018/day2.txt").lines().collect();
    println!("Part 1: {:?}", day02::part1(&input));
    println!("Part 2: {:?}", day02::part2(&input));
}
