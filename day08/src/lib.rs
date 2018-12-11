#[derive(Default, Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn part1_total(&self) -> usize {
        let mut total = 0;
        for metadata in &self.metadata {
            total += metadata;
        }
        for node in &self.children {
            total += node.part1_total();
        }
        total
    }

    fn part2_total(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .map(|m| {
                    self.children
                        .get(m - 1)
                        .map(|node| node.part2_total())
                        .unwrap_or_default()
                })
                .sum()
        }
    }
}

fn as_node(entries: &mut impl Iterator<Item = usize>) -> Node {
    let n_children = entries.next().unwrap();
    let n_metadata = entries.next().unwrap();
    let children = (0..n_children).map(|_| as_node(entries)).collect();
    let metadata = entries.take(n_metadata).collect();
    Node { children, metadata }
}

pub fn part1(input: &str) -> usize {
    let mut entries = input.trim().split(' ').map(|x| x.parse::<usize>().unwrap());
    let node = as_node(&mut entries);
    node.part1_total()
}

pub fn part2(input: &str) -> usize {
    let mut entries = input.trim().split(' ').map(|x| x.parse::<usize>().unwrap());
    let node = as_node(&mut entries);
    node.part2_total()
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(part1(input), 138);
    }

    #[test]
    fn test_part2() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        assert_eq!(part2(input), 66);
    }
}
