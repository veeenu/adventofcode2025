#![feature(portable_simd)]

use std::simd::prelude::*;
use std::time::Instant;

use adventofcode2025::*;

fn parse_int(bytes: &[u8]) -> Option<u64> {
    // TODO simd
    Some(
        bytes
            .iter()
            .rev()
            .enumerate()
            .fold(0u64, |out, (idx, val)| {
                out + ((val - b'0') as u64) * 10u64.pow(idx as u32)
            }),
    )
}

struct NumParser<'a> {
    input: Option<&'a [u8]>,
}

impl<'a> NumParser<'a> {
    fn new(input: &'a str) -> Self {
        NumParser {
            input: Some(input.as_bytes()),
        }
    }
}

impl<'a> Iterator for NumParser<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.input.take()?;
        let next_nl = index_of(input, b'\n')?;

        let num = parse_int(&input[0..next_nl])? as i64;

        self.input = Some(&input[next_nl + 1..]);

        Some(num)
    }
}

struct RangeParser<'a> {
    input: Option<&'a [u8]>,
}

impl<'a> RangeParser<'a> {
    fn new(input: &'a str) -> Self {
        RangeParser {
            input: Some(input.as_bytes()),
        }
    }
}

impl<'a> Iterator for RangeParser<'a> {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.input.take()?;
        let (next_nl, end) = index_of(input, b'\n')
            .map(|i| (i, false))
            .unwrap_or_else(|| (input.len(), true));
        let next_dash = index_of(input, b'-')?;

        let num1 = parse_int(&input[0..next_dash])? as i64;
        let num2 = parse_int(&input[next_dash + 1..next_nl])? as i64;

        self.input = if !end {
            Some(&input[next_nl + 1..])
        } else {
            None
        };

        Some((num1, num2))
    }
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let (ranges, numbers) = input.split_once("\n\n").unwrap();

    let mut ranges = RangeParser::new(ranges).collect::<Vec<_>>();
    let numbers = NumParser::new(numbers).collect::<Vec<_>>();

    ranges.sort_by_key(|(start, _)| *start);

    let mut starts = vec![];
    let mut ends = vec![];

    let (mut current_start, mut current_end) = ranges[0];
    for (next_start, next_end) in ranges.iter().copied().skip(1) {
        if next_start <= current_end + 1 {
            current_end = next_end.max(current_end);
        } else {
            starts.push(current_start);
            ends.push(current_end);
            (current_start, current_end) = (next_start, next_end)
        }
    }

    starts.push(current_start);
    ends.push(current_end);

    (starts, ends, numbers)
}

fn simd_fresh(start: &[i64], end: &[i64], number: i64) -> bool {
    let mut it_s = start.chunks_exact(8);
    let mut it_e = end.chunks_exact(8);

    let n = i64x8::splat(number);

    for (s, e) in (&mut it_s).zip(&mut it_e) {
        let s = i64x8::from_slice(s);
        let e = i64x8::from_slice(e);

        let ss = s.simd_le(n);
        let ee = e.simd_ge(n);

        if (ss & ee).any() {
            return true;
        }
    }

    let (s, e) = (it_s.remainder(), it_e.remainder());
    s.iter()
        .copied()
        .zip(e.iter().copied())
        .any(|(s, e)| number >= s && number <= e)
}

fn part1(input: &str) -> u64 {
    let (starts, ends, numbers) = parse(input);

    numbers
        .into_iter()
        .filter(|&n| simd_fresh(&starts, &ends, n))
        .count() as u64
}

fn simd_count_ranges(start: &[i64], end: &[i64]) -> i64 {
    let mut count = i64x8::splat(0);
    let one = i64x8::splat(1);

    let mut it_s = start.chunks_exact(8);
    let mut it_e = end.chunks_exact(8);

    for (s, e) in (&mut it_s).zip(&mut it_e) {
        let s = i64x8::from_slice(s);
        let e = i64x8::from_slice(e);
        count = count + e - s + one;
    }

    let count = count.to_array().iter().copied().sum::<i64>();
    let rem = it_e
        .remainder()
        .iter()
        .copied()
        .zip(it_s.remainder().iter().copied())
        .map(|(e, s)| e - s + 1)
        .sum::<i64>();

    count + rem
}

fn part2(input: &str) -> u64 {
    let (starts, ends, _) = parse(input);

    simd_count_ranges(&starts, &ends) as u64
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_of() {
        assert_eq!(index_of(b"....@@@@", b'@'), Some(4));
        assert_eq!(index_of(b"....@@@@", b'.'), Some(0));

        let s = b"................................@";
        assert_eq!(index_of(s, b'@'), Some(32));
        assert_eq!(s[32], b'@');

        let s = b".................................@";
        assert_eq!(s[33], b'@');
        assert_eq!(index_of(s, b'@'), Some(33));
    }
}
