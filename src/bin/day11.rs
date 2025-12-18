use std::{
    collections::{HashMap, HashSet, VecDeque},
    time::Instant,
};

use adventofcode2025::get_input;

type Node = [u8; 3];
struct Graph(HashMap<Node, Vec<Node>>);

impl From<&str> for Graph {
    fn from(value: &str) -> Self {
        Graph(
            value
                .lines()
                .map(|line| {
                    let (node, edges) = line.split_once(": ").unwrap();
                    let node: Node = node.as_bytes().try_into().unwrap();
                    let edges: Vec<Node> = edges
                        .split_whitespace()
                        .map(|edge| edge.as_bytes().try_into().unwrap())
                        .collect();

                    (node, edges)
                })
                .collect(),
        )
    }
}

impl Graph {
    fn bfs(&self) -> usize {
        let mut reached = HashSet::new();
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(([b'y', b'o', b'u'], Vec::new()));

        while let Some((node, path)) = q.pop_front() {
            if node == [b'o', b'u', b't'] {
                reached.insert(path);
                continue;
            }

            let edges = self.0.get(&node).unwrap();
            for edge in edges {
                let mut path = path.clone();
                path.push(edge);
                if visited.insert(path.clone()) {
                    q.push_back((*edge, path));
                }
            }
        }

        reached.len()
    }
}

fn part1(input: &str) -> u64 {
    let graph = Graph::from(input);

    graph.bfs() as u64
}

fn part2(_input: &str) -> u64 {
    0
}

fn main() {
    let input = get_input();

    let start = Instant::now();
    let part1 = part1(&input);
    let elapsed = start.elapsed();
    println!("\x1b[32;1mPart 1:\x1b[0m {part1}");
    println!("Took {elapsed:?}");

    let start = Instant::now();
    let part2 = part2(&input);
    let elapsed = start.elapsed();
    println!("\x1b[32;1mPart 2:\x1b[0m {part2}");
    println!("Took {elapsed:?}");
}
