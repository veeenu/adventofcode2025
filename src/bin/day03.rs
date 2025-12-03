use adventofcode2025::get_input;

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let max = line.chars().take(line.len() - 1).max().unwrap();
            let max_idx = line.find(max).unwrap();
            let second_max = line.chars().skip(max_idx + 1).max().unwrap();

            println!("{line} {max}{second_max}");
            let max = max as u8 - b'0';
            let second_max = second_max as u8 - b'0';
            (max * 10 + second_max) as u64
        })
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    fn find_max(line: &str, skip: usize, tail: usize) -> Option<(u8, usize)> {
        let segm = &line[skip..line.len() - tail];
        for digit in ('0'..='9').rev() {
            if let Some(max) = segm.chars().find(|&c| c == digit) {
                let max_idx = segm.find(max).unwrap();

                return Some((max as u8 - b'0', max_idx + skip));
            }
        }

        None
    }

    input
        .lines()
        .map(|line| {
            let mut out = 0u64;
            let mut skip = 0;
            for i in (0..12).rev() {
                let (max, max_idx) = find_max(line, skip, i).unwrap();
                out *= 10;
                out += max as u64;
                skip = max_idx + 1;
            }

            println!("{line} {out}");
            out
        })
        .sum::<u64>()
}

fn main() {
    let input = get_input();

    println!("\x1b[32;1mPart 1:\x1b[0m {}", part1(&input));
    println!("\x1b[32;1mPart 2:\x1b[0m {}", part2(&input));
}
