use std::time::Instant;

use adventofcode2025::get_input;

type Point = (i64, i64);

fn points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(',').map(|b| b.parse::<i64>().unwrap());
            let a = it.next().unwrap();
            let b = it.next().unwrap();
            (a, b)
        })
        .collect()
}

fn area((x1, y1): (i64, i64), (x2, y2): (i64, i64)) -> i64 {
    (y1 - y2 + 1).abs() * (x1 - x2 + 1).abs()
}

fn part1(input: &str) -> u64 {
    let points = points(input);

    let mut areas = points
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(idx, a)| {
            points[idx + 1..]
                .iter()
                .copied()
                .map(move |b| ((a, b), area(a, b)))
        })
        .collect::<Vec<_>>();

    areas.sort_by_key(|&(_, a)| a);

    areas.iter().last().unwrap().1 as u64
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
