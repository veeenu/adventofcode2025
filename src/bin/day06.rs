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

fn simd_operate(operand_columns: Vec<Vec<u64>>, operators: impl Iterator<Item = u64>) -> u64 {
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
    let num_lines = input.lines().count();
    let mut lines = input.lines().rev();
    let first_line = lines.next().unwrap();
    let operators = first_line
        .split_whitespace()
        .map(|c| match c {
            "*" => 1u64,
            "+" => 0u64,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    let acc = operators
        .iter()
        .map(|_| Vec::with_capacity(num_lines))
        .collect::<Vec<_>>();

    let operands = lines
        .rev()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
        })
        .fold(acc, |mut o, row| {
            for (col, i) in o.iter_mut().zip(row) {
                col.push(i);
            }
            o
        });

    simd_operate(operands, operators.into_iter())
}

fn part2(input: &str) -> u64 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();

    let input = input.as_bytes();

    let getxy = |x, y| input[y * (cols + 1) + x];

    let column = |x: usize| (0..rows - 1).map(move |y| getxy(x, y));

    let problems = (0..cols)
        .filter_map(|col| match getxy(col, rows - 1) {
            c @ b'*' | c @ b'+' => {
                let endcol = (col + 1..cols)
                    .find(|&i| {
                        let c = getxy(i, rows - 1);
                        c == b'*' || c == b'+'
                    })
                    .unwrap_or(cols + 1);
                Some((c as char, col, endcol - 1))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    problems
        .into_iter()
        .map(|(op, start_col, end_col)| {
            let it = (start_col..end_col).map(column).map(|col| {
                col.rev()
                    .filter_map(|digit: u8| {
                        if digit.is_ascii_digit() {
                            Some(digit - b'0')
                        } else {
                            None
                        }
                    })
                    .enumerate()
                    .fold(0u64, |o, (idx, i)| o + 10u64.pow(idx as u32) * (i as u64))
            });

            if op == '*' {
                it.product::<u64>()
            } else if op == '+' {
                it.sum::<u64>()
            } else {
                unreachable!()
            }
        })
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
