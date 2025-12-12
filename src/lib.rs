#![feature(portable_simd)]

use std::simd::prelude::*;
use std::{env, fs, path::PathBuf, process::Command};

pub const LANES: usize = 64;

pub fn index_of(bytes: &[u8], val: u8) -> Option<usize> {
    let (head, body, tail) = bytes.as_simd::<LANES>();

    if let Some(pos) = head.iter().position(|&c| c == val) {
        return Some(pos);
    }

    let splat = Simd::splat(val);
    for (idx, chunk) in body.iter().enumerate() {
        if let Some(fs) = chunk.simd_eq(splat).first_set() {
            return Some(head.len() + idx * LANES + fs);
        }
    }

    if let Some(pos) = tail.iter().position(|&c| c == val) {
        return Some(head.len() + body.len() * LANES + pos);
    }

    None
}

pub fn get_input() -> String {
    if let Some(arg) = env::args().nth(1) {
        return fs::read_to_string(arg).expect("Couldn't read input file");
    }

    let exe = env::current_exe().unwrap();
    let exe = exe.file_name().unwrap().to_string_lossy();

    let day = exe.trim_start_matches("day").parse::<u8>().unwrap();

    let path = PathBuf::from(format!("input/day{day:02}.txt"));

    if !path.exists() {
        let url = format!("https://adventofcode.com/2025/day/{day}/input");
        println!("{url}");
        let exit_code = Command::new("curl")
            .arg("--cookie")
            .arg(include_str!("../.cookie"))
            .arg("-L")
            .arg(url)
            .arg("-o")
            .arg(&path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

        if !exit_code.success() {
            panic!("Curl failed: {exit_code}");
        }
    }

    fs::read_to_string(path).unwrap()
}
