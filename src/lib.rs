#![allow(dead_code, unused_imports)]

use std::{
    fs::{self, File},
    io::BufReader,
};
pub fn get_buffer(filename: &str) -> BufReader<File> {
    let f = fs::File::open(filename).unwrap();
    BufReader::new(f)
}

pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
