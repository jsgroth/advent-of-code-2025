use std::error::Error;
use std::fmt::Display;
use std::time::Instant;
use std::{env, fs, hint, io};

pub fn read_input() -> io::Result<String> {
    let mut args = env::args();
    args.next();
    let path = args.next().expect("Missing input path arg");
    fs::read_to_string(path)
}

fn time<T>(input: &str, f: impl Fn(&str) -> T) {
    const RUNS: u128 = 10;

    let mut total_nanos = 0;
    for _ in 0..RUNS {
        let start_time = Instant::now();
        hint::black_box(f(hint::black_box(input)));
        total_nanos += (Instant::now() - start_time).as_nanos();
    }

    let total_millis = total_nanos / 1_000_000 / RUNS;
    println!("{total_millis} ms");
}

pub fn run<T1, T2, F1, F2>(part1: F1, part2: F2) -> Result<(), Box<dyn Error>>
where
    T1: Display,
    T2: Display,
    F1: Fn(&str) -> T1,
    F2: Fn(&str) -> T2,
{
    let input = read_input()?;

    let solution1 = part1(&input);
    println!("{solution1}");

    let solution2 = part2(&input);
    println!("{solution2}");

    if env::var("AOCTIME").is_ok_and(|var| !var.is_empty()) {
        print!("Part 1: ");
        time(&input, part1);

        print!("Part 2: ");
        time(&input, part2);
    }

    Ok(())
}
