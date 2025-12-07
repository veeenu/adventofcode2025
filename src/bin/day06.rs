#![feature(portable_simd)]

use std::simd::prelude::*;
use std::time::Instant;

use adventofcode2025::get_input;

struct Chunks<'a, T: Iterator<Item = u64>> {
    it: &'a mut T,
}

impl<'a, T: Iterator<Item = u64>> Iterator for Chunks<'a, T> {
    type Item = [u64; 8];

    fn next(&mut self) -> Option<Self::Item> {
        let mut a = [0; 8];
        if let Some(v) = self.it.next() {
            a[0] = v;
        } else {
            return None;
        }

        for i in a.iter_mut().skip(1) {
            *i = self.it.next().unwrap_or_default();
        }

        Some(a)
    }
}

fn simd_operate(operand_columns: Vec<Vec<u64>>, operators: Vec<u64>) -> u64 {
    let zero = u64x8::splat(0);

    let mut result_columns = operand_columns
        .iter()
        .zip(operators)
        .map(|(operands, operator)| {
            let muls_or_adds = mask64x8::splat(operator == 1);

            let mut acc_muls = u64x8::splat(1);
            let mut acc_adds = u64x8::splat(0);

            let mut it_n = operands.chunks_exact(8);

            for n in &mut it_n {
                let n = u64x8::from_slice(n);
                acc_muls *= n;
                acc_adds += n;
            }

            let rem = it_n.remainder();
            acc_muls *= u64x8::load_or(rem, u64x8::splat(1));
            acc_adds += u64x8::load_or_default(rem);

            muls_or_adds.select(acc_muls, zero).reduce_product()
                + muls_or_adds.select(zero, acc_adds).reduce_sum()
        });

    let mut acc = u64x8::splat(0);
    let chunks = Chunks {
        it: &mut result_columns,
    };

    for chunk in chunks {
        acc += u64x8::from_slice(&chunk)
    }

    acc.reduce_sum()
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines().rev();
    let operators = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|c| match c {
            "*" => 1u64,
            "+" => 0u64,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let operands = lines
        .rev()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
        })
        .fold(Vec::new(), |mut o, row| {
            if o.is_empty() {
                o.extend(row.map(|i| vec![i]));
            } else {
                for (col, i) in o.iter_mut().zip(row) {
                    col.push(i);
                }
            }
            o
        });

    simd_operate(operands, operators)
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
