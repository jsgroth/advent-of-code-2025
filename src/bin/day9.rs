//! Day 9: Movie Theater
//!
//! <https://adventofcode.com/2025/day/9>
//!
//! Part 1 is trivial, Part 2 very much not so.
//!
//! The basic idea:
//!
//! First, classify each point as an inside corner or an outside corner. This is done by assuming
//! that the leftmost line connects two inside corners, then tracing the loop and inverting
//! inside/outside status each time the line coming into point N is in the same direction as the
//! line going out of point N+1.
//!
//! Next, for each rectangle, check whether each of the 4 rectangle edges is fully contained within
//! the loop.
//!
//! An edge can only leave the loop if it crosses a line in the opposite orientation (horizontal/vertical).
//! The edge always leaves if it crosses in the middle of the line, never leaves if it crosses an
//! outside corner, and sometimes leaves if it crosses an inside corner. For inside corners, the
//! edge leaves the loop if it is not in the same direction as one of the 2 lines touching the corner.

use advent_of_code_2025::Point2D;
use std::cmp;
use std::error::Error;
use std::ops::RangeInclusive;
use winnow::ascii::{digit1, newline};
use winnow::combinator::{opt, separated, separated_pair, terminated};
use winnow::prelude::*;

type Point = Point2D<i64>;

fn parse_i64(input: &mut &str) -> winnow::Result<i64> {
    digit1.parse_to().parse_next(input)
}

fn parse_point(input: &mut &str) -> winnow::Result<Point> {
    let (x, y) = separated_pair(parse_i64, ',', parse_i64).parse_next(input)?;
    Ok(Point { x, y })
}

fn parse_input(input: &mut &str) -> winnow::Result<Vec<Point>> {
    terminated(separated(1.., parse_point, newline), opt(newline)).parse_next(input)
}

fn rectangle_area(point: Point, other_point: Point) -> i64 {
    ((point.x - other_point.x).abs() + 1) * ((point.y - other_point.y).abs() + 1)
}

fn solve_part_1(input: &str) -> i64 {
    let points = parse_input.parse(input).expect("Failed to parse input");

    let mut max = 0;
    for (i, &point) in points.iter().enumerate() {
        for &other_point in &points[i + 1..] {
            let area = rectangle_area(point, other_point);
            max = cmp::max(max, area);
        }
    }

    max
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Corner {
    Inside,  // Inside of corner is inside the loop
    Outside, // Inside of corner is outside the loop
}

impl Corner {
    fn opposite(self) -> Self {
        match self {
            Self::Inside => Self::Outside,
            Self::Outside => Self::Inside,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
    in_vec: Point,  // Vector coming in to start
    out_vec: Point, // Vector going out from end
}

impl Line {
    fn orientation(&self) -> LineOrientation {
        if self.start.x == self.end.x {
            LineOrientation::Vertical
        } else {
            LineOrientation::Horizontal
        }
    }

    fn x_range(&self) -> RangeInclusive<i64> {
        if self.start.x > self.end.x {
            self.end.x..=self.start.x
        } else {
            self.start.x..=self.end.x
        }
    }

    fn y_range(&self) -> RangeInclusive<i64> {
        if self.start.y > self.end.y {
            self.end.y..=self.start.y
        } else {
            self.start.y..=self.end.y
        }
    }
}

#[derive(Debug, Clone)]
struct CorneredLine {
    line: Line,
    start_corner: Corner,
    end_corner: Corner,
}

fn points_to_lines(points: &[Point]) -> Vec<Line> {
    let mut lines = Vec::with_capacity(points.len());

    for i in 0..points.len() {
        let prev_point = if i != 0 { points[i - 1] } else { *points.last().unwrap() };
        let point = points[i];
        let next_point = points[(i + 1) % points.len()];
        let next_next_point = points[(i + 2) % points.len()];

        lines.push(Line {
            start: point,
            end: next_point,
            in_vec: point - prev_point,
            out_vec: next_next_point - next_point,
        });
    }

    lines
}

fn lines_to_cornered(lines: &[Line]) -> Vec<CorneredLine> {
    // Both points on the leftmost vertical line must be inside corners
    let (start_idx, first_vertical_line) = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.orientation() == LineOrientation::Vertical)
        .min_by_key(|(_, line)| line.start.x)
        .unwrap();
    assert_eq!(first_vertical_line.in_vec.x.signum(), -first_vertical_line.out_vec.x.signum());

    let mut cornered = Vec::with_capacity(lines.len());
    cornered.push(CorneredLine {
        line: first_vertical_line.clone(),
        start_corner: Corner::Inside,
        end_corner: Corner::Inside,
    });

    let mut i = (start_idx + 1) % lines.len();
    while i != start_idx {
        let line = &lines[i];
        let start_corner = cornered.last().unwrap().end_corner;
        let end_corner = match line.orientation() {
            LineOrientation::Horizontal if line.in_vec.y.signum() == line.out_vec.y.signum() => {
                start_corner.opposite()
            }
            LineOrientation::Vertical if line.in_vec.x.signum() == line.out_vec.x.signum() => {
                start_corner.opposite()
            }
            _ => start_corner,
        };

        cornered.push(CorneredLine { line: line.clone(), start_corner, end_corner });

        i = (i + 1) % lines.len();
    }

    cornered
}

fn is_fully_inside_loop(
    start: Point,
    end: Point,
    horizontal: &[CorneredLine],
    vertical: &[CorneredLine],
) -> bool {
    check_horizontal_edge(start.y, start.x, end.x, vertical)
        && check_horizontal_edge(end.y, start.x, end.x, vertical)
        && check_vertical_edge(start.x, start.y, end.y, horizontal)
        && check_vertical_edge(end.x, start.y, end.y, horizontal)
}

fn check_horizontal_edge(y: i64, start_x: i64, end_x: i64, vertical: &[CorneredLine]) -> bool {
    check_rectangle_edge(
        y,
        start_x,
        end_x,
        vertical,
        |point| point.x,
        |line| line.line.y_range(),
        |x, y| Point { x, y },
    )
}

fn check_vertical_edge(x: i64, start_y: i64, end_y: i64, horizontal: &[CorneredLine]) -> bool {
    check_rectangle_edge(
        x,
        start_y,
        end_y,
        horizontal,
        |point| point.y,
        |line| line.line.x_range(),
        |y, x| Point { x, y },
    )
}

fn check_rectangle_edge(
    j: i64,
    start_i: i64,
    end_i: i64,
    lines: &[CorneredLine],
    point_i: fn(Point) -> i64,
    line_j_range: fn(&CorneredLine) -> RangeInclusive<i64>,
    make_point: fn(i64, i64) -> Point,
) -> bool {
    let delta = (end_i - start_i).signum();
    if delta == 0 {
        // Edge of length 1; nothing to check
        return true;
    }

    // Find the first occurrence of the start i coordinate
    let line_idx = lines
        .binary_search_by_key(&start_i, |line| point_i(line.line.start))
        .unwrap_or_else(|err| err);
    let mut line_idx = line_idx as i64;
    while (0..lines.len() as i64).contains(&line_idx)
        && point_i(lines[line_idx as usize].line.start) == start_i
    {
        let next = line_idx + delta;
        if !(0..lines.len() as i64).contains(&next)
            || point_i(lines[next as usize].line.start) != start_i
        {
            break;
        }
        line_idx += delta;
    }

    while (0..lines.len() as i64).contains(&line_idx) {
        let line = &lines[line_idx as usize];
        if (end_i - point_i(line.line.start)).signum() != delta.signum() {
            // Reached or passed the end coordinate without leaving the loop
            break;
        }

        if line_j_range(line).contains(&j) {
            // Edge leaves the loop if it passes through an inside corner in the opposite direction
            // of the line leaving that corner OR it passes through the middle of a line
            let point = make_point(point_i(line.line.start), j);
            if line.line.start == point {
                if line.start_corner == Corner::Inside
                    && point_i(line.line.in_vec).signum() == delta.signum()
                {
                    // Passed through inside corner (line start) in the wrong direction
                    return false;
                }
            } else if line.line.end == point {
                if line.end_corner == Corner::Inside
                    && point_i(line.line.out_vec).signum() != delta.signum()
                {
                    // Passed through inside corner (line end) in the wrong direction
                    return false;
                }
            } else {
                // Passed through middle of line
                return false;
            }
        }

        line_idx += delta;
    }

    true
}

fn solve_part_2(input: &str) -> i64 {
    let points = parse_input.parse(input).expect("Failed to parse input");
    let lines = points_to_lines(&points);
    let cornered = lines_to_cornered(&lines);

    let (mut horizontal, mut vertical): (Vec<_>, Vec<_>) = cornered
        .iter()
        .cloned()
        .partition(|line| line.line.orientation() == LineOrientation::Horizontal);
    horizontal.sort_by_key(|line| line.line.start.y);
    vertical.sort_by_key(|line| line.line.start.x);

    let mut max_area = 0;
    for (i, line) in cornered.iter().enumerate() {
        for other_line in &cornered[i + 1..] {
            let a = line.line.start;
            let b = other_line.line.start;
            if is_fully_inside_loop(a, b, &horizontal, &vertical) {
                let area = rectangle_area(a, b);
                max_area = cmp::max(max_area, area);
            }
        }
    }

    max_area
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
 "
        .trim()
    }

    #[test]
    fn part_1() {
        assert_eq!(50, solve_part_1(sample_input()));
    }

    #[test]
    fn part_2() {
        assert_eq!(24, solve_part_2(sample_input()));
    }
}
