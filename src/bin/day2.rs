//! Day 2: Gift Shop
//!
//! <https://adventofcode.com/2025/day/2>

use std::error::Error;

fn invalid_id_part_1(value: &u64) -> bool {
    let s = value.to_string();
    let bytes = s.as_bytes();

    s.len().is_multiple_of(2) && (0..s.len() / 2).all(|i| bytes[i] == bytes[i + s.len() / 2])
}

fn invalid_id_part_2(value: &u64) -> bool {
    let s = value.to_string();
    let bytes = s.as_bytes();

    (1..=s.len() / 2).any(|sub_len| {
        s.len().is_multiple_of(sub_len)
            && (0..sub_len)
                .all(|i| (i + sub_len..s.len()).step_by(sub_len).all(|j| bytes[i] == bytes[j]))
    })
}

fn solve<const PART2: bool>(input: &str) -> u64 {
    let invalid_id_pred = if PART2 { invalid_id_part_2 } else { invalid_id_part_1 };

    input
        .trim()
        .split(',')
        .flat_map(|pair| {
            let (first, second) = pair.split_once('-').expect("Split on '-'");
            let start: u64 = first.trim().parse().expect("Parse start");
            let end: u64 = second.trim().parse().expect("Parse end");
            start..=end
        })
        .filter(invalid_id_pred)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(solve::<false>, solve::<true>)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
"
        .trim()
    }

    #[test]
    fn part_1() {
        assert_eq!(1227775554, solve::<false>(sample_input()));
    }

    #[test]
    fn part_2() {
        assert_eq!(4174379265, solve::<true>(sample_input()));
    }
}
