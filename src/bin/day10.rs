use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use adventofcode2025::get_input;

#[derive(Debug)]
struct Input {
    lights: u32,
    buttons: Vec<u32>,
    joltages: Vec<u32>,
}

fn parse(line: &str) -> Input {
    let (lights, buttons) = line.split_once(' ').unwrap();
    let (buttons, joltages) = buttons.rsplit_once(' ').unwrap();

    let lights = lights
        .trim_start_matches('[')
        .trim_end_matches(']')
        .chars()
        .filter_map(|c| match c {
            '#' => Some(true),
            '.' => Some(false),
            _ => None,
        })
        .enumerate()
        .fold(0u32, |o, (idx, i)| o | if i { 1 << idx } else { 0 });

    let buttons = buttons
        .split_whitespace()
        .map(|btn| {
            btn.trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .fold(0u32, |o, i| {
                    let pow = i.parse::<usize>().unwrap();
                    o | 1 << pow
                })
        })
        .collect::<Vec<_>>();

    let joltages = joltages
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split(',')
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    Input {
        lights,
        buttons,
        joltages,
    }
}

impl Input {
    fn bfs(&self) -> Option<u64> {
        let mut seen = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((0u32, 0u64));

        while let Some((h, count)) = q.pop_front() {
            seen.insert(h);
            if h == self.lights {
                return Some(count);
            }

            for b in &self.buttons {
                let val = h ^ b;
                if !seen.contains(&val) {
                    q.push_back((val, count + 1));
                }
            }
        }

        None
    }

    fn bfs2(&self) -> Option<u64> {
        self.power_set()
            .flat_map(|ops| self.bfs2_inner(&ops))
            .take(100)
            .min()
    }

    fn bfs2_inner(&self, ops: &[Vec<usize>]) -> Option<u64> {
        println!("Opping {ops:?}");
        let mut seen = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(vec![0u32; ops.len()]);

        while let Some(pushes) = q.pop_front() {
            let mut joltages = vec![0u32; self.joltages.len()];

            pushes.iter().zip(ops.iter()).for_each(|(&push, ops)| {
                for &op in ops {
                    joltages[op] += push;
                }
            });

            if joltages == self.joltages {
                let tot_pushes = pushes.iter().sum::<u32>() as u64;
                if !seen.contains(&pushes) {
                    seen.insert(pushes.clone());
                    println!("{pushes:?} x {ops:?} = {joltages:?} ~ {:?}", self.joltages);
                    println!("{tot_pushes} tot");
                    return Some(tot_pushes);
                }
            }

            for i in 0..ops.len() {
                let mut pushes = pushes.clone();
                pushes[i] += 1;

                if joltages
                    .iter()
                    .zip(self.joltages.iter())
                    .all(|(a, b)| a <= b)
                {
                    q.push_back(pushes);
                }
            }
        }

        None
    }

    fn power_set<'a>(self: &'a Input) -> impl Iterator<Item = Vec<Vec<usize>>> + 'a {
        (0..(1usize << self.buttons.len() as u32)).map(|bits| {
            self.buttons
                .iter()
                .enumerate()
                .filter_map(move |(idx, &b)| {
                    if (1 << idx) & bits != 0 {
                        Some(b)
                    } else {
                        None
                    }
                })
                .map(|bits| {
                    (0..16)
                        .filter(|i| ((bits >> i) & 1) == 1)
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>()
        })
    }
}

fn part1(input: &str) -> u64 {
    let lines = input.lines().map(parse).collect::<Vec<_>>();

    lines.iter().filter_map(Input::bfs).sum::<u64>()
}

fn part2(input: &str) -> u64 {
    let lines = input.lines().map(parse).collect::<Vec<_>>();

    lines.iter().filter_map(Input::bfs2).sum::<u64>()
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
