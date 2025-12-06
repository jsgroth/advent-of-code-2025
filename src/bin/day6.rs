//! Day 6: Trash Compactor
//!
//! <https://adventofcode.com/2025/day/6>

use regex::Regex;
use std::error::Error;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Sum,
    Product,
}

impl Operator {
    fn apply(self, values: impl Iterator<Item = u64>) -> u64 {
        match self {
            Self::Sum => values.sum(),
            Self::Product => values.product(),
        }
    }
}

fn parse_operators(last_line: &str) -> Vec<Operator> {
    last_line
        .chars()
        .filter_map(|c| match c {
            '+' => Some(Operator::Sum),
            '*' => Some(Operator::Product),
            _ => None,
        })
        .collect()
}

fn solve_part_1(input: &str) -> u64 {
    static SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" +").unwrap());

    let lines: Vec<_> = input.lines().collect();
    let operators = parse_operators(lines.last().unwrap());

    let operands: Vec<Vec<u64>> = lines[..lines.len() - 1]
        .iter()
        .map(|&line| {
            SPACES
                .split(line)
                .filter(|token| !token.is_empty())
                .map(|operand| operand.parse().unwrap())
                .collect()
        })
        .collect();

    operators
        .into_iter()
        .enumerate()
        .map(|(col, operator)| operator.apply(operands.iter().map(|row| row[col])))
        .sum()
}

fn solve_part_2(input: &str) -> u64 {
    let lines: Vec<_> = input.lines().collect();
    let operators = parse_operators(lines.last().unwrap());

    let digits: Vec<Vec<Option<u64>>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '0'..='9' => Some(c.to_digit(10).unwrap().into()),
                    ' ' => None,
                    _ => panic!("Invalid character {c}"),
                })
                .collect()
        })
        .collect();

    let mut sum = 0;
    let mut end_col = 0;
    for operator in operators {
        let start_col = end_col;
        while digits.iter().any(|row| row.get(end_col).copied().flatten().is_some()) {
            end_col += 1;
        }

        let operands = (start_col..end_col).map(|col| {
            digits
                .iter()
                .filter_map(|row| row.get(col).copied().flatten())
                .fold(0, |acc, digit| 10 * acc + digit)
        });
        sum += operator.apply(operands);

        end_col += 1;
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "
            .trim()
    }

    #[test]
    fn part_1() {
        assert_eq!(4277556, solve_part_1(sample_input()));
    }

    #[test]
    fn part_2() {
        assert_eq!(3263827, solve_part_2(sample_input()));
    }
}
