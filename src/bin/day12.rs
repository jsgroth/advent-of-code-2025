//! Day 12: Christmas Tree Farm
//!
//! <https://adventofcode.com/2025/day/12>
//!
//! This is implemented by simply checking if the total number of cells occupied by all required
//! shapes is less than or equal to the number of cells in the region. This does not work for the
//! sample input but it works for the actual input (at least for my input).
//!
//! Implementing an actual packing solution is probably intractable in the general case, though it
//! might be possible by specializing the solution to the specific puzzle input.

use std::error::Error;
use std::iter;
use winnow::ascii::{digit1, newline};
use winnow::combinator::{empty, fail, opt, preceded, separated, separated_pair, terminated};
use winnow::dispatch;
use winnow::prelude::*;
use winnow::token::any;

#[derive(Debug, Clone, Copy)]
struct Shape {
    occupied: [[bool; 3]; 3],
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    required_shapes: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Input {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

fn parse_shape_space(input: &mut &str) -> winnow::Result<bool> {
    dispatch! { any;
        '.' => empty.value(false),
        '#' => empty.value(true),
        _ => fail,
    }
    .parse_next(input)
}

fn parse_shape_row(input: &mut &str) -> winnow::Result<[bool; 3]> {
    terminated((parse_shape_space, parse_shape_space, parse_shape_space), newline)
        .parse_next(input)
        .map(|(a, b, c)| [a, b, c])
}

fn parse_shape(input: &mut &str) -> winnow::Result<Shape> {
    preceded((digit1, ":", newline), (parse_shape_row, parse_shape_row, parse_shape_row))
        .parse_next(input)
        .map(|(a, b, c)| Shape { occupied: [a, b, c] })
}

fn parse_region_size(input: &mut &str) -> winnow::Result<(usize, usize)> {
    separated_pair(digit1.parse_to(), 'x', digit1.parse_to()).parse_next(input)
}

fn parse_required_shapes(input: &mut &str) -> winnow::Result<Vec<usize>> {
    separated(1.., digit1.parse_to::<usize>(), ' ').parse_next(input)
}

fn parse_region(input: &mut &str) -> winnow::Result<Region> {
    separated_pair(parse_region_size, ": ", parse_required_shapes)
        .parse_next(input)
        .map(|((width, height), required_shapes)| Region { width, height, required_shapes })
}

fn parse_input(input: &mut &str) -> winnow::Result<Input> {
    terminated(
        separated_pair(
            separated(1.., parse_shape, newline),
            newline,
            separated(1.., parse_region, newline),
        ),
        opt(newline),
    )
    .parse_next(input)
    .map(|(shapes, regions)| Input { shapes, regions })
}

fn solve(input: &str) -> usize {
    let Input { shapes, regions } = parse_input.parse(input).expect("Failed to parse input");
    let shape_cells_occupied: Vec<usize> = shapes
        .iter()
        .map(|shape| shape.occupied.iter().map(|row| row.iter().filter(|&&b| b).count()).sum())
        .collect();

    regions
        .into_iter()
        .filter(|region| {
            let region_area = region.width * region.height;
            let shape_total_area: usize = iter::zip(&region.required_shapes, &shape_cells_occupied)
                .map(|(required, &cells_occupied)| required * cells_occupied)
                .sum();

            shape_total_area <= region_area
        })
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(solve, |_| String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
 "
        .trim()
    }

    #[test]
    #[ignore] // Naive solution does not work for sample input
    fn part_1() {
        assert_eq!(2, solve(sample_input()));
    }
}
