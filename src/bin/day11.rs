//! Day 11: Reactor
//!
//! <https://adventofcode.com/2025/day/11>

use rustc_hash::FxHashMap;
use std::error::Error;
use winnow::ascii::{alpha1, newline};
use winnow::combinator::{opt, separated, separated_pair, terminated};
use winnow::prelude::*;

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    edges: Vec<&'a str>,
}

fn parse_edges<'a>(input: &mut &'a str) -> winnow::Result<Vec<&'a str>> {
    separated(1.., alpha1, ' ').parse_next(input)
}

fn parse_node<'a>(input: &mut &'a str) -> winnow::Result<Node<'a>> {
    let (name, edges) = separated_pair(alpha1, ": ", parse_edges).parse_next(input)?;

    Ok(Node { name, edges })
}

fn parse_nodes<'a>(input: &mut &'a str) -> winnow::Result<Vec<Node<'a>>> {
    terminated(separated(1.., parse_node, newline), opt(newline)).parse_next(input)
}

fn solve_part_1(input: &str) -> u64 {
    let nodes = parse_nodes.parse(input).expect("Failed to parse input");
    let node_map: FxHashMap<_, _> = nodes.iter().map(|node| (node.name, node.clone())).collect();

    count_paths("you", &node_map, &mut FxHashMap::default())
}

fn count_paths<'a>(
    name: &'a str,
    node_map: &'a FxHashMap<&str, Node<'_>>,
    cache: &mut FxHashMap<&'a str, u64>,
) -> u64 {
    if name == "out" {
        return 1;
    }

    if let Some(&count) = cache.get(&name) {
        return count;
    }

    let node = node_map.get(&name).expect("Invalid node in input");
    let count = node.edges.iter().map(|&edge| count_paths(edge, node_map, cache)).sum();

    cache.insert(name, count);
    count
}

fn solve_part_2(input: &str) -> u64 {
    let nodes = parse_nodes.parse(input).expect("Failed to parse input");
    let node_map: FxHashMap<_, _> = nodes.iter().map(|node| (node.name, node.clone())).collect();

    count_paths_2("svr", &node_map, false, false, &mut FxHashMap::default())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey<'a> {
    name: &'a str,
    passed_dac: bool,
    passed_fft: bool,
}

fn count_paths_2<'a>(
    name: &'a str,
    node_map: &'a FxHashMap<&str, Node<'_>>,
    passed_dac: bool,
    passed_fft: bool,
    cache: &mut FxHashMap<CacheKey<'a>, u64>,
) -> u64 {
    if name == "out" {
        return if passed_dac && passed_fft { 1 } else { 0 };
    }

    if let Some(&count) = cache.get(&CacheKey { name, passed_dac, passed_fft }) {
        return count;
    }

    let new_passed_dac = passed_dac || name == "dac";
    let new_passed_fft = passed_fft || name == "fft";

    let node = node_map.get(&name).expect("Invalid node in input");
    let count = node
        .edges
        .iter()
        .map(|&edge| count_paths_2(edge, node_map, new_passed_dac, new_passed_fft, cache))
        .sum();

    cache.insert(CacheKey { name, passed_dac, passed_fft }, count);
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
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
 "
        .trim()
    }

    fn sample_input_2() -> &'static str {
        "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"
        .trim()
    }

    #[test]
    fn part_1() {
        assert_eq!(5, solve_part_1(sample_input()));
    }

    #[test]
    fn part_2() {
        assert_eq!(2, solve_part_2(sample_input_2()));
    }
}
