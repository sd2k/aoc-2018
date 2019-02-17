use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;

type Task = char;

struct Edge {
    from: Task,
    to: Task,
}

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        let to = b[5] as Task;
        let from = b[36] as Task;
        Ok(Edge { to, from })
    }
}

#[derive(Debug, Default)]
struct Node {
    blocks: BTreeSet<Task>,
    blocked_by: BTreeSet<Task>,
}

type Graph = HashMap<Task, Node>;

fn new_graph(edges: &[Edge]) -> Graph {
    let mut graph = Graph::new();
    for edge in edges {
        graph.entry(edge.to).or_default().blocks.insert(edge.from);
        graph
            .entry(edge.from)
            .or_default()
            .blocked_by
            .insert(edge.to);
    }
    graph
}

fn process_graph(
    graph: &Graph,
    mut available: BTreeSet<char>,
    mut done: BTreeSet<char>,
    mut done_v: String,
) -> String {
    if available.is_empty() {
        return done_v;
    } else {
        let next = available.iter().next().unwrap().clone();
        available.remove(&next);
        done.insert(next);
        done_v.push(next);
        let node = &graph[&next];
        for potential in &node.blocks {
            if graph[&potential].blocked_by.is_subset(&done) {
                available.insert(*potential);
            }
        }
        return process_graph(graph, available, done, done_v);
    }
}

pub fn part1(input: &[&str]) -> String {
    let edges: Vec<Edge> = input.iter().map(|l| Edge::from_str(l).unwrap()).collect();
    let graph = new_graph(&edges);

    let available: BTreeSet<char> = graph
        .iter()
        .filter(|(_, node)| node.blocked_by.is_empty())
        .map(|(k, _)| k)
        .cloned()
        .collect();

    process_graph(&graph, available, BTreeSet::new(), String::new())
}

pub fn part2(input: &[&str], n_workers: usize, base_duration: usize) -> String {
    let edges: Vec<Edge> = input.iter().map(|l| Edge::from_str(l).unwrap()).collect();
    let graph = new_graph(&edges);

    let mut total_time = 0;
    let total_tasks = graph.len();

    // workers will hold a vector of tasks currently being worked on,
    // and their remaining durations.
    let mut workers: Vec<Option<(char, usize)>> = vec![None; n_workers];

    let mut done: BTreeSet<char> = BTreeSet::new();
    let mut available: BTreeSet<char> = graph
        .iter()
        .filter(|(_, node)| node.blocked_by.is_empty())
        .map(|(k, _)| k)
        .cloned()
        .collect();

    while done.len() < total_tasks {
        // There may be duplicates here - that's OK.
        let fastest_current_task = workers.iter().filter_map(|x| *x).min_by_key(|x| x.1);
        match fastest_current_task {
            // This will be the case after the first iteration, once
            // we've added the initial tasks.
            Some(fastest_task) => {
                total_time += fastest_task.1;

                // Now check if any of the workers have finished their tasks.
                for mut worker in &mut workers {
                    match &mut worker {
                        // If so, remove the task from them, add the task to
                        // the 'done' tasks, and insert any newly available
                        // tasks.
                        Some(current_task) if current_task.1 == fastest_task.1 => {
                            done.insert(current_task.0);

                            let task = &graph[&current_task.0];
                            for potential in &task.blocks {
                                if graph[&potential].blocked_by.is_subset(&done) {
                                    available.insert(*potential);
                                }
                            }
                            *worker = None;
                        }
                        // If not (but they're working on a task), decrease
                        // the remaining duration.
                        Some(ref mut current_task) => {
                            current_task.1 -= fastest_task.1;
                        }
                        // If they're not working on anything then do nothing.
                        None => {}
                    }
                }
            }
            None => {}
        }

        // Now that we've determined the available tasks, assign the idle
        // workers the available tasks in order, removing that task
        // from the 'available' set.
        for worker in &mut workers {
            if worker.is_none() {
                let next = available.iter().next().cloned();
                match next {
                    Some(t) => {
                        available.remove(&t);
                        *worker = Some((t, base_duration + (t as u8 - 64) as usize));
                    }
                    None => {}
                }
            }
        }
    }
    total_time.to_string()
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

    #[test]
    fn test_part2() {
        let input = &[
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ];
        assert_eq!(part2(input, 2, 0), "15");
    }
}
