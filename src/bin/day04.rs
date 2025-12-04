#![feature(portable_simd)]

use std::{simd::prelude::*, time::Instant};

use adventofcode2025::get_input;
type SimdVec = Simd<u8, 32>;

// fn index_of(bytes: &[u8], val: u8) -> Option<usize> {
//     let mut it = bytes.chunks(32);
//
//     let vals = SimdVec::splat(val);
//     let mut idx = 0;
//
//     for chunk in &mut it {
//         let chunk = SimdVec::load_or_default(chunk);
//         if let Some(fs) = vals.simd_eq(chunk).first_set() {
//             return Some(idx + fs);
//         }
//         idx += 32;
//     }
//
//     None
// }

fn to_mask(input: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; input.len()];
    let mut it_in = input.chunks_exact(32);
    let mut it_out = out.chunks_exact_mut(32);

    let ats = SimdVec::splat(b'@');
    let ones = SimdVec::splat(1);
    let zeros = SimdVec::splat(0);

    for (out_chunk, in_chunk) in (&mut it_out).zip(&mut it_in) {
        let v = SimdVec::load_or_default(in_chunk)
            .simd_eq(ats)
            .select(ones, zeros);
        v.copy_to_slice(out_chunk);
    }

    let v = SimdVec::load_or_default(it_in.remainder())
        .simd_eq(ats)
        .select(ones, zeros);
    let mut buf = [0u8; 32];
    v.copy_to_slice(&mut buf);
    let rem = it_out.into_remainder();
    rem.copy_from_slice(&buf[..rem.len()]);

    out
}

fn compute_neighborhood_y(input: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; input.len()];
    let mut i = 1;

    out[0] = input[0] + input[1];

    while i + 32 < input.len() {
        let left = SimdVec::load_or_default(&input[i - 1..]);
        let center = SimdVec::load_or_default(&input[i..]);
        let right = SimdVec::load_or_default(&input[i + 1..]);
        let neigh = left + right + center;

        neigh.copy_to_slice(&mut out[i..]);

        i += 32;
    }

    let left = SimdVec::load_or_default(&input[i - 1..]);
    let center = SimdVec::load_or_default(&input[i..]);
    let right = SimdVec::load_or_default(&input[i + 1..]);
    let neigh = left + right + center;

    let o = &mut out[i..];
    let mut v = [0u8; 32];
    neigh.copy_to_slice(&mut v[..]);
    o.copy_from_slice(&v[..o.len()]);

    out
}

fn compute_neighborhood_x(input: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; input.len()];
    let mut i = 1;

    out[0] = input[1];

    while i + 32 < input.len() {
        let left = SimdVec::load_or_default(&input[i - 1..]);
        let right = SimdVec::load_or_default(&input[i + 1..]);
        let neigh = left + right;

        neigh.copy_to_slice(&mut out[i..]);

        i += 32;
    }

    let left = SimdVec::load_or_default(&input[i - 1..]);
    let right = SimdVec::load_or_default(&input[i + 1..]);
    let neigh = left + right;

    let o = &mut out[i..];
    let mut v = [0u8; 32];
    neigh.copy_to_slice(&mut v[..]);
    o.copy_from_slice(&v[..o.len()]);

    out
}

fn addv(out_v: &mut [u8], in_v: &[u8]) {
    let mut it_in = in_v.chunks_exact(32);
    let mut it_out = out_v.chunks_exact_mut(32);

    for (i, o) in (&mut it_in).zip(&mut it_out) {
        let ii = SimdVec::load_or_default(i);
        let oo = SimdVec::load_or_default(o);
        (ii + oo).copy_to_slice(o);
    }

    let rem = it_out.into_remainder();
    let ii = SimdVec::load_or_default(it_in.remainder());
    let oo = SimdVec::load_or_default(rem);
    let mut v = [0u8; 32];
    (ii + oo).copy_to_slice(&mut v);
    rem.copy_from_slice(&v[..rem.len()]);
}

fn mulv(out_v: &mut [u8], in_v: &[u8]) {
    let mut it_in = in_v.chunks_exact(32);
    let mut it_out = out_v.chunks_exact_mut(32);

    for (i, o) in (&mut it_in).zip(&mut it_out) {
        let ii = SimdVec::load_or_default(i);
        let oo = SimdVec::load_or_default(o);
        let vv: SimdVec = ii * oo;

        vv.copy_to_slice(o);
    }

    let rem = it_out.into_remainder();
    let ii = SimdVec::load_or_default(it_in.remainder());
    let oo = SimdVec::load_or_default(rem);
    let vv: SimdVec = ii * oo;
    let mut v = [0u8; 32];
    vv.copy_to_slice(&mut v);
    rem.copy_from_slice(&v[..rem.len()]);
}

fn lt(out_v: &mut [u8]) {
    let zero = SimdVec::splat(0);
    let one = SimdVec::splat(1);
    let cmp = SimdVec::splat(4);

    let mut it_out = out_v.chunks_exact_mut(32);

    for chunk in &mut it_out {
        let v = SimdVec::load_or_default(chunk);
        let r = v.simd_lt(cmp).select(one, zero);

        r.copy_to_slice(chunk);
    }

    let rem = it_out.into_remainder();
    let v = SimdVec::load_or_default(rem);
    let r = v.simd_lt(cmp).select(one, zero);
    let mut v = [0u8; 32];
    r.copy_to_slice(&mut v);
    rem.copy_from_slice(&v[..rem.len()]);
}

fn compute_neighborhood(before: Option<&[u8]>, current: &[u8], after: Option<&[u8]>) -> Vec<u8> {
    let mut out = vec![0u8; current.len()];

    if let Some(before) = before {
        addv(&mut out, &compute_neighborhood_y(before));
    }

    if let Some(after) = after {
        addv(&mut out, &compute_neighborhood_y(after));
    }

    addv(&mut out, &compute_neighborhood_x(current));

    out
}

fn compute_accessible(before: Option<&[u8]>, current: &[u8], after: Option<&[u8]>) -> Vec<u8> {
    let mut out = compute_neighborhood(before, current, after);

    lt(&mut out);
    mulv(&mut out, current);
    out
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

    #[test]
    fn test_to_mask() {
        assert_eq!(to_mask(b"@.@.@."), vec![1, 0, 1, 0, 1, 0]);
    }

    #[test]
    fn test_neighborhood() {
        let s = b"..@@.@@@@.";
        assert_eq!(
            compute_neighborhood_x(&to_mask(s)),
            vec![0, 1, 1, 1, 2, 1, 2, 2, 1, 1]
        );
        assert_eq!(
            compute_neighborhood_y(&to_mask(s)),
            vec![0, 1, 2, 2, 2, 2, 3, 3, 2, 1]
        );
    }
}

fn compute_accessible_all(input: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut output = Vec::with_capacity(input.len());

    output.push(compute_accessible(None, &input[0], Some(&input[1])));

    for i in 1..input.len() - 1 {
        output.push(compute_accessible(
            Some(&input[i - 1]),
            &input[i],
            Some(&input[i + 1]),
        ));
    }

    output.push(compute_accessible(
        Some(&input[input.len() - 2]),
        &input[input.len() - 1],
        None,
    ));

    output
}

fn count_accessible(input: &[Vec<u8>]) -> u64 {
    input
        .iter()
        .map(|row| row.iter().map(|&i| i as u64).sum::<u64>())
        .sum::<u64>()
}

fn remove(output: &mut [Vec<u8>], removed: &[Vec<u8>]) {
    for (output, removed) in output.iter_mut().zip(removed.iter()) {
        for (chunk_out, chunk_rem) in output.chunks_mut(32).zip(removed.chunks(32)) {
            let out = SimdVec::load_or_default(chunk_out);
            let rem = SimdVec::load_or_default(chunk_rem);
            let rem = SimdVec::splat(1) - rem;
            let out = out * rem;
            if chunk_out.len() == 32 {
                out.copy_to_slice(chunk_out);
            } else {
                let mut v = [0u8; 32];
                out.copy_to_slice(&mut v);
                chunk_out.copy_from_slice(&v[..chunk_out.len()]);
            }
        }
    }
}

fn print_map(input: &[Vec<u8>]) {
    for row in input {
        for &col in row {
            if col == 1 { print!("@") } else { print!(".") }
        }
        println!();
    }
}

fn part1(input: &str) -> u64 {
    let input = input
        .lines()
        .map(|l| to_mask(l.as_bytes()))
        .collect::<Vec<_>>();

    let accessible = compute_accessible_all(&input);
    count_accessible(&accessible)
}

fn part2(input: &str) -> u64 {
    let input = input
        .lines()
        .map(|l| to_mask(l.as_bytes()))
        .collect::<Vec<_>>();

    let mut output = input.to_vec();
    let mut removed_count = 0;

    loop {
        let removed = compute_accessible_all(&output);
        let accessible_count = count_accessible(&removed);
        removed_count += accessible_count;

        remove(&mut output, &removed);

        // println!();
        // print_map(&output);
        // println!("{accessible_count}");
        if accessible_count == 0 {
            break;
        }
    }

    removed_count
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
