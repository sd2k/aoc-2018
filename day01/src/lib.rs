use std::collections::HashSet;

fn parse_element(el: &str) -> i64 {
    el.trim_left_matches('+').parse().unwrap()
}

pub fn part1(input: &[&str]) -> i64 {
    input.iter().map(|el| parse_element(el)).sum()
}

pub fn part2_functional(input: &[&str]) -> i64 {
    let mut seen = HashSet::new();
    seen.insert(0);
    let err: Result<_, i64> = input.iter().cycle().try_fold(0, |acc, &x| {
        let new = acc + parse_element(x);
        if seen.contains(&new) {
            return Err(new);
        } else {
            seen.insert(new);
            return Ok(new);
        }
    });
    return err.err().unwrap();
}

pub fn part2_imperative(input: &[&str]) -> i64 {
    let mut current = 0;
    let mut seen = HashSet::new();
    seen.insert(current);
    let repeated = input.iter().cycle();
    for element in repeated {
        current = current + parse_element(element);
        if seen.contains(&current) {
            return current;
        }
        seen.insert(current);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {

    use super::{part1, part2_functional, part2_imperative};

    #[test]
    fn test_part1() {
        assert_eq!(part1(&["+1", "+1", "+1"]), 3);
        assert_eq!(part1(&["+1", "+1", "-2"]), 0);
        assert_eq!(part1(&["-1", "-2", "-3"]), -6);
    }

    #[test]
    fn test_part2_functional() {
        assert_eq!(part2_functional(&["+1", "-1"]), 0);
        assert_eq!(part2_functional(&["+3", "+3", "+4", "-2", "-4"]), 10);
        assert_eq!(part2_functional(&["-6", "+3", "+8", "+5", "-6"]), 5);
        assert_eq!(part2_functional(&["+7", "+7", "-2", "-7", "-4"]), 14);
    }

    #[test]
    fn test_part2_imperative() {
        assert_eq!(part2_imperative(&["+1", "-1"]), 0);
        assert_eq!(part2_imperative(&["+3", "+3", "+4", "-2", "-4"]), 10);
        assert_eq!(part2_imperative(&["-6", "+3", "+8", "+5", "-6"]), 5);
        assert_eq!(part2_imperative(&["+7", "+7", "-2", "-7", "-4"]), 14);
    }
}
