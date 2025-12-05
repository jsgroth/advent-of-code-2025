//! Day 5: Cafeteria
//!
//! <https://adventofcode.com/2025/day/5>

use std::cmp;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    fn contains(self, value: u64) -> bool {
        (self.start..=self.end).contains(&value)
    }
}

fn parse_ranges<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<IdRange> {
    lines
        .map(|line| {
            let (start, end) = line.split_once('-').expect("Split range");
            let start: u64 = start.parse().expect("Parse start");
            let end: u64 = end.parse().expect("Parse end");
            IdRange { start, end }
        })
        .collect()
}

fn solve_part_1(input: &str) -> usize {
    let mut lines = input.lines();

    let ranges = parse_ranges(lines.by_ref().take_while(|line| !line.is_empty()));

    lines
        .filter(|line| {
            let value: u64 = line.parse().expect("Parse ingredient ID");
            ranges.iter().any(|&range| range.contains(value))
        })
        .count()
}

fn solve_part_2(input: &str) -> u64 {
    let mut ranges = parse_ranges(input.lines().take_while(|line| !line.is_empty()));

    ranges.sort_by_key(|range| range.start);

    let mut i = 0;
    while i < ranges.len() - 1 {
        if ranges[i + 1].start <= ranges[i].end {
            ranges[i].end = cmp::max(ranges[i].end, ranges[i + 1].end);
            ranges.remove(i + 1);
            continue;
        }

        i += 1;
    }

    ranges.into_iter().map(|range| range.end - range.start + 1).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"
        .trim()
    }

    #[test]
    fn part_1() {
        assert_eq!(3, solve_part_1(sample_input()));
    }

    #[test]
    fn part_2() {
        assert_eq!(14, solve_part_2(sample_input()));
    }
}
