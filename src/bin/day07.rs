#![feature(portable_simd)]

use std::simd::{LaneCount, SupportedLaneCount, prelude::*};
use std::time::Instant;

use adventofcode2025::*;

// TODO: so close yet so far
fn compute_splits<const LANES: usize>(
    beams_current: &[u8],
    row: &[u8],
    beams_next: &mut [u8],
) -> usize
where
    LaneCount<LANES>: SupportedLaneCount,
{
    let mut beams_current = beams_current.chunks(LANES);
    let mut row = row.chunks(LANES);
    let mut beams_next = beams_next.chunks_mut(LANES);

    let ones = Simd::<u8, LANES>::splat(1);
    let zeros = Simd::<u8, LANES>::splat(0);
    let all_beams = Simd::<u8, LANES>::splat(b'|');
    let all_dots = Simd::<u8, LANES>::splat(b'.');
    let all_splitters = Simd::<u8, LANES>::splat(b'^');
    let submask = all_beams - all_splitters;

    let mut carries = Mask::<i8, LANES>::splat(false);
    carries.set(LANES - 1, true);
    let mut carries_back = Mask::<i8, LANES>::splat(false);
    carries_back.set(0, true);

    let mut carry = false;
    let mut carry_back = false;

    let mut split_count = 0;

    for ((current, row), next) in (&mut beams_current).zip(&mut row).zip(&mut beams_next) {
        let current = Simd::<u8, LANES>::from_slice(current);
        let row = Simd::<u8, LANES>::from_slice(row);

        // Split positions: will not have a beam next
        let splits = row.simd_eq(current - submask);

        // Beam positions: will have a beam if unimpeded
        let beams = current.simd_eq(all_beams);
        // Split outputs: will have a beam next
        let split_outputs =
            splits.shift_elements_left::<1>(carry_back) | splits.shift_elements_right::<1>(carry);

        carry = (carries & splits).any();
        carry_back = (carries_back & splits).any();

        split_count += (beams & splits).select(ones, zeros).reduce_sum();

        let beams_next = beams & (!splits) | split_outputs;
        let beams_next = beams_next.select(all_beams, all_dots);

        beams_next.copy_to_slice(next);
    }

    split_count as usize
}

fn compute_splits_nonsimd(beams_current: &[u8], row: &[u8], beams_next: &mut [u8]) -> usize {
    let mut split_count = 0;

    for (idx, (beam, maybe_split)) in beams_current
        .iter()
        .copied()
        .zip(row.iter().copied())
        .enumerate()
    {
        match (beam, maybe_split) {
            (b'|', b'^') => {
                split_count += 1;
                beams_next[idx - 1..=idx + 1].copy_from_slice(b"|.|");
            }
            (b'|', _) => beams_next[idx] = b'|',
            _ => {}
        }
    }

    split_count
}

fn part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let width = index_of(input, b'\n').unwrap();
    // let width_pad = width.next_power_of_two();
    let mut beams_current = vec![b'.'; width];
    let mut beams_next = vec![b'.'; width];
    let mut row = vec![b'.'; width];

    let start_beam = index_of(input, b'S').unwrap();
    beams_current[start_beam] = b'|';

    let mut pos = width + 1;
    let mut split_count = 0;
    loop {
        row[..width].copy_from_slice(&input[pos..pos + width]);
        let sc = compute_splits_nonsimd(&beams_current, &row, &mut beams_next);
        split_count += sc;

        beams_current.copy_from_slice(&beams_next);

        pos += width + 1;
        if pos >= input.len() - 1 {
            break;
        }
    }

    split_count as u64
}

fn compute_timelines_nonsimd(beams_current: &[u64], row: &[u8], beams_next: &mut [u64]) {
    for idx_pass in row
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(idx, c)| if c == b'.' { Some(idx) } else { None })
    {
        beams_next[idx_pass] = beams_current[idx_pass];
    }

    for idx_split in row
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(idx, c)| if c == b'^' { Some(idx) } else { None })
    {
        beams_next[idx_split - 1] += beams_current[idx_split];
        beams_next[idx_split] = 0;
        beams_next[idx_split + 1] += beams_current[idx_split];
    }
}

fn part2(input: &str) -> u64 {
    let input = input.as_bytes();

    let width = index_of(input, b'\n').unwrap();
    let mut beams_current = vec![0u64; width];
    let mut beams_next = vec![0u64; width];
    let mut row = vec![0u8; width];

    let start_beam = index_of(input, b'S').unwrap();
    beams_current[start_beam] = 1;

    let mut pos = width + 1;
    loop {
        row[..width].copy_from_slice(&input[pos..pos + width]);
        compute_timelines_nonsimd(&beams_current, &row, &mut beams_next);
        beams_current.copy_from_slice(&beams_next);

        pos += width + 1;
        if pos >= input.len() - 1 {
            break;
        }
    }

    beams_current.iter().sum::<u64>()
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
