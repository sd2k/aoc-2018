use std::collections::{HashMap, HashSet};
use std::str::FromStr;

struct Dependency(char, char);
impl FromStr for Dependency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        let dep = b[5] as char;
        let name = b[36] as char;
        Ok(Dependency(name, dep))
    }
}

pub fn part1(input: &[&str]) -> String {
    let dependencies: HashMap<char, Vec<char>> = input
        .iter()
        .map(|line| Dependency::from_str(line).unwrap())
        .fold(HashMap::new(), |mut acc, dep| {
            acc.entry(dep.0)
                .and_modify(|e| {
                    e.push(dep.1);
                })
                .or_insert(vec![dep.1]);
            acc
        });
    let reverse_dependencies: HashMap<char, Vec<char>> = input
        .iter()
        .map(|line| Dependency::from_str(line).unwrap())
        .fold(HashMap::new(), |mut acc, dep| {
            acc.entry(dep.1)
                .and_modify(|e| {
                    e.push(dep.0);
                })
                .or_insert(vec![dep.0]);
            acc
        });
    let tasks_with_deps = dependencies.keys().cloned().collect::<HashSet<char>>();
    let deps_of_tasks = reverse_dependencies
        .keys()
        .cloned()
        .collect::<HashSet<char>>();
    let mut current_task = deps_of_tasks.difference(&tasks_with_deps).next().unwrap();
    let mut available = HashSet::new();
    let mut tasks_in_order = Vec::with_capacity(reverse_dependencies.len());
    loop {
        if let Some(deps) = dependencies.get(current_task) {
            for dep in deps {
                available.insert(dep);
            }
        }
        let available = available.clone();
        if let Some(new_task) = available.iter().min() {
            tasks_in_order.push(new_task);
            current_task = new_task;
        } else {
            break;
        }
    }
    tasks_in_order.into_iter().cloned().collect()
}

pub fn part2(input: &[&str]) -> String {
    unimplemented!()
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = &[
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ];
        assert_eq!(part1(input), "CABDFE");
    }

    // #[test]
    // fn test_part2() {
    //     let input = &[];
    //     assert_eq!(part2(input), _);
    // }
}
