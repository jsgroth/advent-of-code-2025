use std::error::Error;
use std::fmt::Display;
use std::{env, fs, io};

pub fn read_input() -> io::Result<String> {
    let mut args = env::args();
    args.next();
    let path = args.next().expect("Missing input path arg");
    fs::read_to_string(path)
}

pub fn run<T1, T2, F1, F2>(part1: F1, part2: F2) -> Result<(), Box<dyn Error>>
where
    T1: Display,
    T2: Display,
    F1: FnOnce(&str) -> T1,
    F2: FnOnce(&str) -> T2,
{
    let input = read_input()?;

    let solution1 = part1(&input);
    println!("{solution1}");

    let solution2 = part2(&input);
    println!("{solution2}");

    Ok(())
}
