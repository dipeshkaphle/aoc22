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
pub mod day2;
pub mod day3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
