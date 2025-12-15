use std::time::Instant;

use adventofcode2025::get_input;

type Point = (i64, i64, i64);

fn points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(',').map(|b| b.parse::<i64>().unwrap());
            let a = it.next().unwrap();
            let b = it.next().unwrap();
            let c = it.next().unwrap();
            (a, b, c)
        })
        .collect()
}

fn distance((x1, y1, z1): (i64, i64, i64), (x2, y2, z2): (i64, i64, i64)) -> f32 {
    (((x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2)) as f32).sqrt()
}

fn merge(a: Point, b: Point, point_sets: &mut [Vec<Point>]) {
    let a_idx = point_sets.iter().position(|set| set.contains(&a)).unwrap();
    let b_idx = point_sets.iter().position(|set| set.contains(&b)).unwrap();

    if a_idx == b_idx {
        return;
    }

    let (a_idx, b_idx) = (a_idx.min(b_idx), a_idx.max(b_idx));
    let (left, right) = point_sets.split_at_mut(a_idx + 1);
    left[a_idx].append(&mut right[b_idx - a_idx - 1]);
}

fn part1(input: &str) -> u64 {
    let points = points(input);
    let mut distances = points
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(idx, a)| {
            points[idx + 1..]
                .iter()
                .copied()
                .map(move |b| ((a, b), distance(a, b)))
        })
        .collect::<Vec<_>>();

    distances.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let mut point_sets = points.iter().copied().map(|p| vec![p]).collect::<Vec<_>>();

    for &((a, b), _) in &distances[..points.len()] {
        merge(a, b, &mut point_sets);
    }

    point_sets.sort_by_key(|a| a.len());

    point_sets
        .into_iter()
        .rev()
        .take(3)
        .fold(1, |o, i| o * i.len()) as u64
}

fn part2(input: &str) -> u64 {
    let points = points(input);
    let mut distances = points
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(idx, a)| {
            points[idx + 1..]
                .iter()
                .copied()
                .map(move |b| ((a, b), distance(a, b)))
        })
        .collect::<Vec<_>>();

    distances.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    let mut point_sets = points.iter().copied().map(|p| vec![p]).collect::<Vec<_>>();

    distances
        .iter()
        .find_map(|&((a, b), _)| {
            merge(a, b, &mut point_sets);
            point_sets.retain(|s| !s.is_empty());
            if point_sets.len() == 1 {
                Some(a.0 * b.0)
            } else {
                None
            }
        })
        .unwrap_or_default() as u64
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
