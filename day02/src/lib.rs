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

pub fn part2(_input: &[&str]) -> Vec<char> {
    unimplemented!()
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
        assert_eq!(part2(input), vec!['f', 'g', 'i', 'j']);
    }
}
