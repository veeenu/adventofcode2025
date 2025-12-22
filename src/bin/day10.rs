use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use adventofcode2025::get_input;
use rayon::prelude::*;
use z3::{Solver, ast::Int};

#[derive(Debug)]
struct Input {
    lights: u32,
    buttons: Vec<Vec<i32>>,
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
                .filter_map(|i| i.parse::<i32>().ok())
                .collect::<Vec<_>>()
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
        let buttons = self
            .buttons
            .iter()
            .map(|buttons| buttons.iter().copied().fold(0u32, |o, i| o | 1 << i))
            .collect::<Vec<_>>();

        let mut seen = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((0u32, 0u64));

        while let Some((h, count)) = q.pop_front() {
            seen.insert(h);
            if h == self.lights {
                return Some(count);
            }

            for b in &buttons {
                let val = h ^ b;
                if !seen.contains(&val) {
                    q.push_back((val, count + 1));
                }
            }
        }

        None
    }

    fn solve(&self) -> Option<u64> {
        let presses: Vec<Int> = [
            "zero",
            "one",
            "two",
            "three",
            "four",
            "five",
            "six",
            "seven",
            "eight",
            "nine",
            "ten",
            "eleven",
            "twelve",
            "thirteen",
            "fourteen",
            "fifteen",
            "sixteen",
            "seventeen",
            "eighteen",
            "nineteen",
            "twenty",
        ]
        .iter()
        .take(self.buttons.len())
        .copied()
        .map(Int::fresh_const)
        .collect::<Vec<_>>();

        let solver = Solver::new();
        for (col, joltage) in self.joltages.iter().copied().enumerate() {
            let affecting_buttons = self
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(idx, button)| {
                    if button.contains(&(col as i32)) {
                        Some(presses[idx].clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            solver.assert(Int::add(&affecting_buttons).eq(joltage));
        }

        for press in &presses {
            solver.assert(press.ge(0));
        }

        solver
            .solutions(presses, false)
            .map(|sol| sol.iter().filter_map(Int::as_u64).sum::<u64>())
            .min()
    }
}

fn part1(input: &str) -> u64 {
    let lines = input.lines().map(parse).collect::<Vec<_>>();

    lines.iter().filter_map(Input::bfs).sum::<u64>()
}

fn part2(input: &str) -> u64 {
    let lines = input.lines().map(parse).collect::<Vec<_>>();

    lines
        .par_iter()
        .map(|s: &Input| s.solve())
        .inspect(|i| println!("{i:?}"))
        .flatten()
        .sum::<u64>()
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
