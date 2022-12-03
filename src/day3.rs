use std::{collections::HashSet, io::BufRead};

use crate::get_buffer;

struct Compartment {
    c1: String,
    c2: String,
}

pub fn get_common(s1: &str, s2: &str) -> Vec<char> {
    let dict = s2.chars().into_iter().collect::<HashSet<char>>();
    s1.chars()
        .filter(|x| dict.contains(x))
        .collect::<HashSet<char>>()
        .into_iter()
        .collect::<Vec<char>>()
}
pub fn get_score(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - ('A' as usize) + 1 + 26
    } else {
        c as usize - ('a' as usize) + 1
    }
}

pub struct Day3 {}
impl Day3 {
    fn parse() -> Vec<String> {
        let buf = get_buffer("input/day3.txt");
        buf.lines().map(|x| x.unwrap()).collect()
    }

    pub fn part_1() -> usize {
        let inp = Day3::parse();
        inp.iter()
            .map(|x| Compartment {
                c1: x[0..(x.len() >> 1)].to_owned(),
                c2: x[(x.len() >> 1)..].to_owned(),
            })
            .map(|x| get_common(&x.c1, &x.c2))
            .map(|x| x.iter().map(|y| get_score(*y)).sum::<usize>())
            .sum()
    }

    pub fn part_2() -> usize {
        let inp = Day3::parse();
        inp.chunks(3)
            .map(|x| {
                let common = get_common(x.get(0).unwrap().as_ref(), x.get(1).unwrap().as_ref())
                    .into_iter()
                    .collect::<String>();
                get_common(x.get(2).unwrap().as_ref(), &common)
                    .iter()
                    .map(|y| get_score(*y))
                    .sum::<usize>()
            })
            .sum()
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day3;
        println!("{:?}", Day3::part_1());
        println!("{:?}", Day3::part_2());
    }
}
