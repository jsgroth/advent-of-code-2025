//! Day 1: Secret Entrance
//!
//! <https://adventofcode.com/2025/day/1>

use std::error::Error;
use std::str::FromStr;

fn solve_part_1(input: &str) -> u32 {
    let mut position = 50;
    let mut count = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut rotation = i32::from_str(&line[1..]).expect("Invalid line");
        if line.as_bytes()[0] == b'L' {
            rotation *= -1;
        }

        position = (position + rotation).rem_euclid(100);
        if position == 0 {
            count += 1;
        }
    }

    count
}

fn solve_part_2(input: &str) -> u32 {
    let mut position: i32 = 50;
    let mut count = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let magnitude = i32::from_str(&line[1..]).expect("Invalid line");

        let direction = match line.as_bytes()[0] {
            b'L' => -1,
            b'R' => 1,
            _ => panic!("Invalid line {line}"),
        };

        for _ in 0..magnitude {
            position = (position + direction).rem_euclid(100);
            if position == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
        .trim()
    }

    #[test]
    fn part_1() {
        assert_eq!(3, solve_part_1(sample_input()));
    }

    #[test]
    fn part_2() {
        assert_eq!(6, solve_part_2(sample_input()));
    }
}
