enum Candidate {
    None,
    Two,
    Three,
    Both,
}

fn check_word(word: &&str) -> Candidate {
    // Use an array of counts rather than having to do hash lookups
    // or anything silly, since we know IDs are all lower case letters
    let initial = [0; 26];
    let counts = word.chars().fold(initial, |mut acc, c| {
        // 'a' is 97 in ascii, so subtracting 97 gets us its index (zero)
        // Similarly, this gets us an index of 25 for 'z'.
        let index: usize = c as usize - 97;
        acc[index] += 1;
        acc
    });
    let any_threes = counts.iter().any(|&x| x == 3);
    let any_twos = counts.iter().any(|&x| x == 2);
    match (any_twos, any_threes) {
        (true, true) => Candidate::Both,
        (true, false) => Candidate::Two,
        (false, true) => Candidate::Three,
        (false, false) => Candidate::None,
    }
}

#[derive(Default)]
struct Counts {
    two: u64,
    three: u64,
}

pub fn part1(input: &[&str]) -> u64 {
    let mut counts = Counts::default();
    for candidate in input.iter().map(check_word) {
        match candidate {
            Candidate::Both => {
                counts.two += 1;
                counts.three += 1;
            }
            Candidate::Three => counts.three += 1,
            Candidate::Two => counts.two += 1,
            _ => {}
        }
    }
    counts.two * counts.three
}

enum StrDiff {
    One(usize),
    More,
}

fn str_diff(a: &str, b: &str) -> StrDiff {
    let diffs: Vec<usize> = a
        .chars()
        .enumerate()
        .zip(b.chars())
        .filter(|((_, a), b)| a != b)
        .map(|((i, _), _)| i)
        .collect();
    match diffs.len() {
        1 => StrDiff::One(diffs[0]),
        _ => StrDiff::More,
    }
}

pub fn part2(input: &[&str]) -> String {
    for s1 in input.iter() {
        for s2 in input.iter().skip(1) {
            match str_diff(s1, s2) {
                StrDiff::One(idx) => {
                    return s1
                        .chars()
                        .enumerate()
                        .filter(|(i, _)| i != &idx)
                        .map(|(_, c)| c)
                        .collect()
                }
                StrDiff::More => {}
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = &[
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(part1(input), 12);
    }

    #[test]
    fn test_part2() {
        let input = &[
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        assert_eq!(part2(input), "fgij");
    }
}
