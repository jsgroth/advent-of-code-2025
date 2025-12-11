//! Day 10: Factory
//!
//! <https://adventofcode.com/2025/day/10>
//!
//! This solution is absolutely terrible but it finishes in less than 30 seconds (by using multiple
//! threads).
//!
//! I'm sure there's a fancy linear algebra solution to this problem, but this solves it using
//! a search+pruning approach.

use rustc_hash::FxHashSet;
use std::cmp::Reverse;
use std::collections::VecDeque;
use std::error::Error;
use std::str::FromStr;
use std::{cmp, thread};
use winnow::ascii::{digit1, newline};
use winnow::combinator::{delimited, empty, fail, opt, repeat, separated, terminated};
use winnow::dispatch;
use winnow::prelude::*;
use winnow::token::any;

#[derive(Debug, Clone)]
struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage_reqs: Vec<i32>,
}

fn parse_num<T: FromStr>(input: &mut &str) -> winnow::Result<T> {
    digit1.parse_to().parse_next(input)
}

fn parse_indicator(input: &mut &str) -> winnow::Result<bool> {
    dispatch! { any;
        '.' => empty.value(false),
        '#' => empty.value(true),
        _ => fail,
    }
    .parse_next(input)
}

fn parse_button(input: &mut &str) -> winnow::Result<Vec<usize>> {
    delimited('(', separated(1.., parse_num::<usize>, ','), ')').parse_next(input)
}

fn parse_joltage_reqs(input: &mut &str) -> winnow::Result<Vec<i32>> {
    delimited('{', separated(1.., parse_num::<i32>, ','), '}').parse_next(input)
}

fn parse_machine(input: &mut &str) -> winnow::Result<Machine> {
    let indicators = delimited('[', repeat(1.., parse_indicator), ']').parse_next(input)?;
    ' '.parse_next(input)?;
    let buttons = separated(1.., parse_button, ' ').parse_next(input)?;
    ' '.parse_next(input)?;
    let joltage_reqs = parse_joltage_reqs.parse_next(input)?;

    Ok(Machine { indicators, buttons, joltage_reqs })
}

fn parse_input(input: &mut &str) -> winnow::Result<Vec<Machine>> {
    terminated(separated(1.., parse_machine, newline), opt(newline)).parse_next(input)
}

struct QueueEntry {
    state: Vec<bool>,
    len: u32,
}

fn solve_part_1(input: &str) -> u32 {
    let machines = parse_input.parse(input).expect("Failed to parse input");

    machines
        .into_iter()
        .map(|machine| {
            let mut queue = VecDeque::new();
            queue.push_back(QueueEntry { state: vec![false; machine.indicators.len()], len: 0 });

            let mut visited = FxHashSet::default();
            visited.insert(vec![false; machine.indicators.len()]);

            while let Some(QueueEntry { state, len }) = queue.pop_front() {
                for button in &machine.buttons {
                    let mut new_state = state.clone();
                    for &indicator in button {
                        new_state[indicator] = !new_state[indicator];
                    }

                    if new_state == machine.indicators {
                        return len + 1;
                    }

                    if !visited.contains(&new_state) {
                        visited.insert(new_state.clone());
                        queue.push_back(QueueEntry { state: new_state, len: len + 1 });
                    }
                }
            }

            panic!("Queue emptied without reaching the target indicator state")
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Equation {
    indices: Vec<usize>,
    joltage_idx: usize,
}

fn generate_equations(machine: &Machine) -> Vec<Equation> {
    let mut equations: Vec<_> = machine
        .joltage_reqs
        .iter()
        .enumerate()
        .map(|(joltage_idx, _)| Equation { indices: Vec::new(), joltage_idx })
        .collect();

    for (i, button) in machine.buttons.iter().enumerate() {
        for &idx in button {
            equations[idx].indices.push(i);
        }
    }

    equations
}

fn solve_part_2(input: &str) -> u32 {
    let machines = parse_input.parse(input).expect("Failed to parse input");

    let mut threads = Vec::new();
    for machine in machines {
        threads.push(thread::spawn(move || {
            let mut equations = generate_equations(&machine);
            equations.sort_by_key(|equation| equation.indices.len());
            for equation in &mut equations {
                equation
                    .indices
                    .sort_by_key(|&button_idx| Reverse(machine.buttons[button_idx].len()));
            }

            let mut min_presses = machine.joltage_reqs.iter().copied().sum::<i32>() as u32;
            find_solutions(
                0,
                machine.buttons.clone(),
                &machine.joltage_reqs,
                &equations,
                &mut min_presses,
            );

            min_presses
        }));
    }

    threads.into_iter().map(|thread| thread.join().unwrap()).sum()
}

fn find_solutions(
    presses: u32,
    buttons: Vec<Vec<usize>>,
    joltages: &[i32],
    equations: &[Equation],
    min_presses: &mut u32,
) {
    if equations.is_empty() {
        *min_presses = cmp::min(*min_presses, presses);
        return;
    }

    if presses + joltages.iter().copied().max().unwrap() as u32 >= *min_presses {
        return;
    }

    let button_indices: Vec<_> = equations[0]
        .indices
        .iter()
        .copied()
        .filter(|&button_idx| !buttons[button_idx].is_empty())
        .collect();
    if button_indices.is_empty() {
        if joltages[equations[0].joltage_idx] == 0 {
            find_solutions(presses, buttons, joltages, &equations[1..], min_presses);
        }
        return;
    }

    test_button(&button_indices, presses, &buttons, joltages, equations, min_presses);
}

fn test_button(
    button_indices: &[usize],
    presses: u32,
    buttons: &[Vec<usize>],
    joltages: &[i32],
    equations: &[Equation],
    min_presses: &mut u32,
) {
    if button_indices.is_empty() {
        let mut new_buttons = buttons.to_vec();
        for &button_idx in &equations[0].indices {
            new_buttons[button_idx].clear();
        }

        find_solutions(presses, new_buttons, joltages, &equations[1..], min_presses);
        return;
    }

    let button_idx = button_indices[0];

    let mut min_possible = 0;
    if button_indices.len() == 1 {
        min_possible = joltages[equations[0].joltage_idx];

        for &joltage_idx in &buttons[button_idx] {
            if let Some(equation) =
                equations[1..].iter().find(|equation| equation.joltage_idx == joltage_idx)
                && equation.indices.iter().all(|&button_idx| {
                    buttons[button_idx].is_empty() || equations[0].indices.contains(&button_idx)
                })
            {
                min_possible = cmp::max(min_possible, joltages[equation.joltage_idx]);
            }
        }
    }

    let max_possible =
        buttons[button_idx].iter().map(|&joltage_idx| joltages[joltage_idx]).min().unwrap_or(0);
    let max_possible = cmp::min(max_possible, (*min_presses as i32) - (presses as i32) - 1);

    for button_presses in (min_possible..=max_possible).rev() {
        if presses + (button_presses as u32) >= *min_presses {
            break;
        }

        let new_presses = presses + button_presses as u32;
        let mut new_joltages = joltages.to_vec();

        for &joltage_idx in &buttons[button_idx] {
            new_joltages[joltage_idx] -= button_presses;
            assert!(new_joltages[joltage_idx] >= 0);
        }

        test_button(
            &button_indices[1..],
            new_presses,
            buttons,
            &new_joltages,
            equations,
            min_presses,
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    advent_of_code_2025::run(solve_part_1, solve_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
 "
        .trim()
    }

    #[test]
    fn part_1() {
        assert_eq!(7, solve_part_1(sample_input()));
    }

    #[test]
    fn part_2() {
        assert_eq!(33, solve_part_2(sample_input()));
    }
}
