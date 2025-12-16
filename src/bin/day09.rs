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
    ((y1 - y2).abs() + 1) * ((x1 - x2).abs() + 1)
}

fn part1(input: &str) -> u64 {
    let points = points(input);

    points
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(idx, a)| points[idx + 1..].iter().copied().map(move |b| area(a, b)))
        .max()
        .unwrap() as u64
}

fn rect_in_poly((x1, y1): Point, (x2, y2): Point, points: &[Point]) -> bool {
    let (x1, x2) = (x1.min(x2), x1.max(x2));
    let (y1, y2) = (y1.min(y2), y1.max(y2));

    let mut i = 0;
    let mut j = points.len() - 1;

    while i < points.len() {
        let (xi, yi) = points[i];
        let (xj, yj) = points[j];

        if xi > x1 && xi < x2 && yi > y1 && yi < y2 {
            return false;
        }

        let (ya, yb) = (yi.min(yj), yi.max(yj));
        if xi == xj && x1 < xi && x2 > xi && ya <= y1 && yb >= y2 {
            return false;
        }

        let (xa, xb) = (xi.min(xj), xi.max(xj));
        if yi == yj && y1 < yi && y2 > yi && xa <= x1 && xb >= x2 {
            return false;
        }

        j = i;
        i += 1;
    }

    true
}

fn part2(input: &str) -> u64 {
    let points = points(input);

    points
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(idx, a)| {
            points[idx + 1..]
                .iter()
                .copied()
                .map(move |b| (a, b, area(a, b)))
        })
        .filter_map(|(p1, p2, a)| {
            if rect_in_poly(p1, p2, &points) {
                Some(a)
            } else {
                None
            }
        })
        .max()
        .unwrap() as u64
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
