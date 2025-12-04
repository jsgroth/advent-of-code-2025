//! Day 4: Printing Department
//!
//! <https://adventofcode.com/2025/day/4>

use std::error::Error;

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn count_neighbors(grid: &[Vec<bool>], i: usize, j: usize) -> u32 {
    let mut neighbors = 0;

    for di in -1..=1 {
        for dj in -1..=1 {
            if di == 0 && dj == 0 {
                continue;
            }

            let ii = (i as i32) + di;
            let jj = (j as i32) + dj;
            if (0..grid.len() as i32).contains(&ii)
                && (0..grid[i].len() as i32).contains(&jj)
                && grid[ii as usize][jj as usize]
            {
                neighbors += 1;
            }
        }
    }

    neighbors
}

fn solve_part_1(input: &str) -> usize {
    let grid = parse_input(input);

    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, &occupied)| occupied && count_neighbors(&grid, i, j) < 4)
                .count()
        })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    let mut grid = parse_input(input);

    let mut removed = 0;
    loop {
        let mut modified = false;

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] && count_neighbors(&grid, i, j) < 4 {
                    grid[i][j] = false;
                    modified = true;
                    removed += 1;
                }
            }
        }

        if !modified {
            break;
        }
    }

    removed
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
        .trim()
    }

    #[test]
    fn part_1() {
        assert_eq!(13, solve_part_1(sample_input()));
    }

    #[test]
    fn part_2() {
        assert_eq!(43, solve_part_2(sample_input()));
    }
}
