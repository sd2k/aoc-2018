#[inline(always)]
fn compare_elements(x: char, y: char) -> bool {
    // My version
    // x != y && x.to_ascii_lowercase() == y.to_ascii_lowercase()
    // Optimization taken from Reddit:
    // https://www.reddit.com/r/adventofcode/comments/a3912m/2018_day_5_solutions/eb4ilyz/
    x as u8 ^ 32 == y as u8
}

pub fn part1(input: &str) -> usize {
    input
        .chars()
        .fold(Vec::new(), |mut acc, x| {
            match acc.last() {
                Some(prev) if compare_elements(x, *prev) => {
                    acc.pop();
                }
                _ => {
                    acc.push(x);
                }
            }
            acc
        })
        .len()
}

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn part2(input: &str) -> usize {
    LETTERS
        .chars()
        .map(|l| {
            part1(
                &input
                    .chars()
                    .filter(|c| c.to_ascii_lowercase() != l)
                    .collect::<String>(),
            )
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};
    use env_logger;

    #[test]
    fn test_part1() {
        env_logger::init();
        assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
        assert_eq!(part1("aabAAB"), 6);
        assert_eq!(part1("aA"), 0);
        assert_eq!(part1("abBA"), 0);
        assert_eq!(part1("abAB"), 4);
        assert_eq!(
            part1("iITJoOyRrYiIXxjgLlGrRnSsShHsNqQdNngGDwNHhDgGWwaAjSsJdnLlOogG"),
            2
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("dabAcCaCBAcCcaDA"), 4);
    }
}
