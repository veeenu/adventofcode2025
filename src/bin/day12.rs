use std::time::Instant;

use adventofcode2025::get_input;

fn part1(input: &str) -> u64 {
    let (tiles, problems) = input.trim().rsplit_once("\n\n").unwrap();

    let tiles = tiles
        .trim()
        .split("\n\n")
        .map(|tile| {
            let b = tile.as_bytes();
            [
                b[3] as char,
                b[4] as char,
                b[5] as char,
                b[7] as char,
                b[8] as char,
                b[9] as char,
                b[11] as char,
                b[12] as char,
                b[13] as char,
            ]
        })
        .collect::<Vec<_>>();

    let problems = problems
        .trim()
        .lines()
        .map(|problem| {
            let (area, counts) = problem.split_once(": ").unwrap();
            let (w, h) = area.split_once('x').unwrap();
            let (w, h) = (w.parse::<i32>().unwrap(), h.parse::<i32>().unwrap());
            let counts = counts
                .split_whitespace()
                .filter_map(|c| c.parse::<i32>().ok())
                .collect::<Vec<_>>();

            (w, h, counts)
        })
        .collect::<Vec<_>>();

    println!("{tiles:?}\n{problems:?}");

    problems
        .into_iter()
        .filter(|(w, h, counts)| {
            let upper_bound = counts.iter().sum::<i32>() * 9;
            let lower_bound = counts
                .iter()
                .enumerate()
                .map(|(idx, &count)| {
                    tiles[idx].iter().filter(|&&c| c == '#').count() as i32 * count
                })
                .sum::<i32>();

            println!(
                "{w},{h},{counts:?} -> {} < {} < {}",
                lower_bound,
                w * h,
                upper_bound
            );

            if w * h >= upper_bound {
                println!("will always fit (ub)",);
                true
            } else if w * h < lower_bound {
                println!("will never fit (lb)");
                false
            } else {
                todo!("may not fit");
            }
        })
        .count() as u64
}

fn main() {
    let input = get_input();

    let start = Instant::now();
    let part1 = part1(&input);
    let elapsed = start.elapsed();
    println!("\x1b[32;1mPart 1:\x1b[0m {part1}");
    println!("Took {elapsed:?}");
}
