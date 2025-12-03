//! Day 3: Lobby
//!
//! <https://adventofcode.com/2025/day/3>

use std::error::Error;

const PART_1_BATTERIES: usize = 2;
const PART_2_BATTERIES: usize = 12;

fn solve(input: &str, batteries_len: usize) -> u64 {
    input
        .lines()
        .map(|line| {
            let batteries: Vec<_> = line.as_bytes().iter().map(|&c| u64::from(c - b'0')).collect();
            find_max_joltage(&batteries, batteries_len)
        })
        .sum()
}

fn solve_part_1(input: &str) -> u64 {
    solve(input, PART_1_BATTERIES)
}

fn solve_part_2(input: &str) -> u64 {
    solve(input, PART_2_BATTERIES)
}

fn find_max_joltage(batteries: &[u64], len: usize) -> u64 {
    assert!(len <= batteries.len());

    if len == 0 {
        return 0;
    }

    let end = batteries.len() - len + 1;

    let mut max_digit = 0;
    let mut max_digit_idx = 0;
    for (i, &digit) in batteries.iter().enumerate().take(end) {
        if digit > max_digit {
            max_digit = digit;
            max_digit_idx = i;
        }
    }

    let sub_max = find_max_joltage(&batteries[max_digit_idx + 1..], len - 1);
    sub_max + batteries[max_digit_idx] * 10_u64.pow((len - 1) as u32)
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../../sample/day3.txt");

    #[test]
    fn part_1() {
        assert_eq!(357, solve_part_1(SAMPLE_INPUT));
    }

    #[test]
    fn part_2() {
        assert_eq!(3121910778619, solve_part_2(SAMPLE_INPUT));
    }
}
