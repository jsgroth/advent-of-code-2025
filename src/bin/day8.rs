//! Day 8: Playground
//!
//! <https://adventofcode.com/2025/day/8>

use advent_of_code_2025::Point3D;
use std::cmp::Reverse;
use std::error::Error;
use std::{cmp, mem};
use winnow::ascii::{digit1, newline};
use winnow::combinator::{opt, separated, terminated};
use winnow::prelude::*;

type Point = Point3D<i64>;

const REAL_CONNECTIONS: usize = 1000;

fn parse_i64(input: &mut &str) -> winnow::Result<i64> {
    digit1.parse_to().parse_next(input)
}

fn parse_point(input: &mut &str) -> winnow::Result<Point> {
    let (x, _, y, _, z) = (parse_i64, ',', parse_i64, ',', parse_i64).parse_next(input)?;
    Ok(Point { x, y, z })
}

fn parse_input(input: &mut &str) -> winnow::Result<Vec<Point>> {
    terminated(separated(1.., parse_point, newline), opt(newline)).parse_next(input)
}

trait PointExt {
    fn euclidean_distance_squared_to(self, other: Self) -> i64;
}

impl PointExt for Point {
    fn euclidean_distance_squared_to(self, other: Self) -> i64 {
        let delta = self - other;
        delta.x * delta.x + delta.y * delta.y + delta.z * delta.z
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PointWithIndex {
    p: Point,
    idx: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Connection {
    distance_squared: i64,
    a: PointWithIndex,
    b: PointWithIndex,
}

struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    max_size: usize,
}

impl UnionFind {
    fn new(len: usize) -> Self {
        let mut parents = vec![0; len];
        for (i, parent) in parents.iter_mut().enumerate() {
            *parent = i;
        }

        Self { parents, sizes: vec![1; len], max_size: 1 }
    }

    fn find(&mut self, mut i: usize) -> usize {
        while self.parents[i] != i {
            let t = self.parents[i];
            self.parents[i] = self.parents[self.parents[i]];
            i = t;
        }

        i
    }

    fn union(&mut self, mut i: usize, mut j: usize) {
        i = self.find(i);
        j = self.find(j);

        if i == j {
            return;
        }

        if self.sizes[i] < self.sizes[j] {
            mem::swap(&mut i, &mut j);
        }

        self.parents[j] = i;
        self.sizes[i] += self.sizes[j];
        self.max_size = cmp::max(self.max_size, self.sizes[i]);
    }
}

fn connections_sorted_by_distance(points: &[Point]) -> Vec<Connection> {
    let mut connections = Vec::with_capacity(points.len() * points.len() / 2);

    for (i, &point) in points.iter().enumerate() {
        for (j, &other_point) in points[i + 1..].iter().enumerate() {
            let distance_squared = point.euclidean_distance_squared_to(other_point);
            connections.push(Connection {
                distance_squared,
                a: PointWithIndex { p: point, idx: i },
                b: PointWithIndex { p: other_point, idx: j + i + 1 },
            });
        }
    }

    connections.sort_by_key(|connection| connection.distance_squared);
    connections
}

fn solve_part_1(input: &str, num_connections: usize) -> usize {
    let points = parse_input.parse(input).expect("Failed to parse input");
    let connections = connections_sorted_by_distance(&points);

    let mut circuits = UnionFind::new(points.len());
    for connection in &connections[..num_connections] {
        circuits.union(connection.a.idx, connection.b.idx);
    }

    circuits.sizes.sort_by_key(|&size| Reverse(size));
    circuits.sizes[..3].iter().copied().product()
}

fn solve_part_2(input: &str) -> i64 {
    let points = parse_input.parse(input).expect("Failed to parse input");
    let connections = connections_sorted_by_distance(&points);

    let mut circuits = UnionFind::new(points.len());
    for connection in connections {
        circuits.union(connection.a.idx, connection.b.idx);

        if circuits.max_size == points.len() {
            return connection.a.p.x * connection.b.p.x;
        }
    }

    panic!("Circuit never fully connected; should be impossible")
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(|input| solve_part_1(input, REAL_CONNECTIONS), solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CONNECTIONS: usize = 10;

    fn sample_input() -> &'static str {
        "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"
        .trim()
    }

    #[test]
    fn part_1() {
        assert_eq!(40, solve_part_1(sample_input(), SAMPLE_CONNECTIONS));
    }

    #[test]
    fn part_2() {
        assert_eq!(25272, solve_part_2(sample_input()));
    }
}
