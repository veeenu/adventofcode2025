use std::time::Instant;

use adventofcode2025::get_input;

fn part1(_input: &str) -> u64 {
    0
}

fn is_invalid(&num: &u64) -> bool {
    let log = (num as f32).log10() as u32 + 1;

    for pow in 1..=(log / 2) {
        let modulus = 10u64.pow(pow);
        let base = num % modulus;
        if base == 0 || base < 10u64.pow(pow - 1) {
            continue;
        }
        let mut acc = 0u64;
        while acc < num {
            acc *= modulus;
            acc += base;
        }
        if acc == num {
            println!("{num} is invalid!");
            return true;
        }
    }

    false
}

fn part2(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .flat_map(|range| {
            let (l, r) = dbg!(range.split_once('-').unwrap());
            let l = l.parse::<u64>().unwrap();
            let r = r.parse::<u64>().unwrap();

            (l..=r).filter(is_invalid)
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
