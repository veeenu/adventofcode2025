use std::{
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
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
    fn bfs(&self) -> u64 {
        let mut reached = HashSet::new();
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((*b"you", Vec::new()));

        while let Some((node, path)) = q.pop_front() {
            if &node == b"out" {
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

        reached.len() as u64
    }

    fn dfs2(&self) -> u64 {
        let svr_dac = self.dfs2_inner(*b"svr", *b"dac", &mut HashMap::new());
        let dac_fft = self.dfs2_inner(*b"dac", *b"fft", &mut HashMap::new());
        let fft_out = self.dfs2_inner(*b"fft", *b"out", &mut HashMap::new());

        let svr_fft = self.dfs2_inner(*b"svr", *b"fft", &mut HashMap::new());
        let fft_dac = self.dfs2_inner(*b"fft", *b"dac", &mut HashMap::new());
        let dac_out = self.dfs2_inner(*b"dac", *b"out", &mut HashMap::new());

        svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
    }

    fn dfs2_inner(&self, start: Node, end: Node, memo: &mut HashMap<Node, u64>) -> u64 {
        if let Some(&cache) = memo.get(&start) {
            return cache;
        }

        if start == end {
            return 1;
        }

        let mut paths = 0u64;
        if let Some(edges) = self.0.get(&start) {
            for &edge in edges {
                paths += self.dfs2_inner(edge, end, memo);
            }
        }

        memo.insert(start, paths);

        paths
    }
}

fn part1(input: &str) -> u64 {
    Graph::from(input).bfs()
}

fn part2(input: &str) -> u64 {
    Graph::from(input).dfs2()
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
