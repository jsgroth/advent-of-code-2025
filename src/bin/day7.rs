//! Day 7: Laboratories
//!
//! <https://adventofcode.com/2025/day/7>

use advent_of_code_2025::{Grid2D, Point2D};
use std::collections::VecDeque;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    Splitter,
}

struct Input {
    grid: Grid2D<Space>,
    start: Point2D<usize>,
}

fn parse_input(input: &str) -> Input {
    let mut start: Option<Point2D<usize>> = None;

    let grid: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '.' => Space::Empty,
                    '^' => Space::Splitter,
                    'S' => {
                        start = Some(Point2D { x: col, y: row });
                        Space::Empty
                    }
                    _ => panic!("Invalid character {c}"),
                })
                .collect()
        })
        .collect();

    Input { grid: Grid2D(grid), start: start.expect("No start in input") }
}

fn solve_part_1(input: &str) -> u64 {
    let Input { grid, start } = parse_input(input);

    let mut visited = Grid2D(vec![vec![false; grid.cols()]; grid.rows()]);
    visited[start] = true;

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut splitters_hit = 0;
    while let Some(point) = queue.pop_front() {
        let mut maybe_enqueue_point = |point: Point2D<usize>| {
            if point.y < grid.rows() && !visited[point] {
                visited[point] = true;
                queue.push_back(point);
            }
        };

        match grid[point] {
            Space::Empty => {
                maybe_enqueue_point(point + Point2D { x: 0, y: 1 });
            }
            Space::Splitter => {
                splitters_hit += 1;
                maybe_enqueue_point(point + Point2D { x: 1, y: 0 });
                maybe_enqueue_point(point - Point2D { x: 1, y: 0 });
            }
        }
    }

    splitters_hit
}

fn solve_part_2(input: &str) -> u64 {
    let Input { grid, start } = parse_input(input);
    let mut cache = Grid2D(vec![vec![None; grid.cols()]; grid.rows()]);
    count_timelines(&grid, start, &mut cache)
}

fn count_timelines(
    grid: &Grid2D<Space>,
    point: Point2D<usize>,
    cache: &mut Grid2D<Option<u64>>,
) -> u64 {
    if point.y == grid.rows() {
        return 1;
    }

    if let Some(timelines) = cache[point] {
        return timelines;
    }

    let timelines = match grid[point] {
        Space::Empty => count_timelines(grid, point + Point2D { x: 0, y: 1 }, cache),
        Space::Splitter => {
            count_timelines(grid, point + Point2D { x: 1, y: 0 }, cache)
                + count_timelines(grid, point - Point2D { x: 1, y: 0 }, cache)
        }
    };

    cache[point] = Some(timelines);
    timelines
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
 "
        .trim()
    }

    #[test]
    fn part_1() {
        assert_eq!(21, solve_part_1(sample_input()));
    }

    #[test]
    fn part_2() {
        assert_eq!(40, solve_part_2(sample_input()));
    }
}
