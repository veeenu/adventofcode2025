use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use adventofcode2025::get_input;

#[derive(Debug)]
struct Input {
    lights: u32,
    buttons: Vec<u32>,
}

fn parse(line: &str) -> Input {
    let mut it = line.split_whitespace();
    let lights_s = it
        .next()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']');
    let lights = lights_s
        .chars()
        .filter_map(|c| match c {
            '#' => Some(true),
            '.' => Some(false),
            _ => None,
        })
        .enumerate()
        .fold(0u32, |o, (idx, i)| o | if i { 1 << idx } else { 0 });

    println!("{lights_s} -> {lights:012b}");

    let buttons = it
        .take_while(|s| s.starts_with('('))
        .map(|btn| {
            let v = btn
                .trim_start_matches('(')
                .trim_end_matches(')')
                .split(',')
                .fold(0u32, |o, i| {
                    let pow = i.parse::<usize>().unwrap();
                    o | 1 << pow
                });

            println!("{btn} -> {v:010b}");
            v
        })
        .collect::<Vec<_>>();

    Input { lights, buttons }
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
}

fn part1(input: &str) -> u64 {
    let lines = input.lines().map(parse).collect::<Vec<_>>();

    // for line in lines {
    //     println!("{:08b}", line.lights);
    //     for op in &line.buttons {
    //         print!("{op:08b} ");
    //     }
    //     println!("\n");
    //
    //     println!("{:?}: {:?}", line, line.bfs());
    // }

    println!("{}", lines.len());
    lines.iter().filter_map(Input::bfs).sum::<u64>()
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
