use std::error::Error;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Point2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T: AddAssign> AddAssign for Point2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Point2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T: SubAssign> SubAssign for Point2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Add<Output = T>> Add for Point3D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<T: AddAssign> AddAssign for Point3D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub<Output = T>> Sub for Point3D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<T: SubAssign> SubAssign for Point3D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[derive(Debug, Clone)]
pub struct Grid2D<T>(pub Vec<Vec<T>>);

impl<T> Grid2D<T> {
    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn cols(&self) -> usize {
        self.0[0].len()
    }
}

macro_rules! impl_grid_index {
    ($t:ty) => {
        impl<T> Index<Point2D<$t>> for Grid2D<T> {
            type Output = T;

            fn index(&self, index: Point2D<$t>) -> &Self::Output {
                &self.0[index.y as usize][index.x as usize]
            }
        }

        impl<T> IndexMut<Point2D<$t>> for Grid2D<T> {
            fn index_mut(&mut self, index: Point2D<$t>) -> &mut Self::Output {
                &mut self.0[index.y as usize][index.x as usize]
            }
        }
    };
}

impl_grid_index!(usize);
impl_grid_index!(i32);
