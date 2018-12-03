use std::str::FromStr;

use hashbrown::HashMap;
use lazy_static::lazy_static;
use regex::{self, Regex};

const CLAIM_REGEX: &str = r"#(?P<id>\d+) @ (?P<x>\d+),(?P<y>\d+): (?P<w>\d+)x(?P<h>\d+)";

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Square(u32, u32);

#[derive(Debug)]
struct Claim {
    id: String,
    squares: Vec<Square>,
}

impl FromStr for Claim {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(CLAIM_REGEX).unwrap();
        }
        let caps = RE.captures(s).expect("claim must be in expected format");
        let left = caps["x"].parse::<u32>().expect("x coord must be a number");
        let right = left + caps["w"].parse::<u32>().expect("width must be a number");
        let top = caps["y"].parse::<u32>().expect("y coord must be a number");
        let bottom = top + caps["h"].parse::<u32>().expect("height must be a number");
        Ok(Claim {
            id: caps["id"].to_string(),
            squares: (left..right)
                .flat_map(move |x| (top..bottom).map(move |y| Square(x, y)))
                .collect(),
        })
    }
}

// Convert each claim to a list of occupied squares, then create
// a HashMap from Square to count, and find any with a count > 1.
pub fn part1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|x| Claim::from_str(x).expect("could not parse claim"))
        .fold(HashMap::new(), |mut acc, claim| {
            for rect in claim.squares {
                acc.entry(rect).and_modify(|e| *e += 1).or_insert(1);
            }
            acc
        })
        .iter()
        .filter(|(_, v)| **v > 1)
        .count()
}

// Determine the number of claims per rectangle, then look for
// claims whose rectangles only have a single claim.
pub fn part2(input: &[&str]) -> String {
    let claims: Vec<Claim> = input.iter().map(|x| Claim::from_str(x).unwrap()).collect();
    let claims_per_rectangle =
        claims
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<Square, usize>, claim| {
                for rect in &claim.squares {
                    acc.entry(rect.clone()).and_modify(|e| *e += 1).or_insert(1);
                }
                acc
            });
    claims
        .iter()
        .filter_map(|claim| {
            // Check if any squares have more than one claim.
            // If not, return the ID.
            match claim
                .squares
                .iter()
                .filter(|rect| claims_per_rectangle.get(rect).unwrap() != &1)
                .count()
            {
                0 => Some(claim.id.clone()),
                _ => None,
            }
        })
        .next()
        .expect("a valid claim")
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = &["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part2() {
        let input = &["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2"];
        assert_eq!(part2(input), "3");
    }
}
