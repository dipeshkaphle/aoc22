use std::{collections::HashMap, io::BufRead};

use crate::get_buffer;

#[derive(Debug)]
pub struct Instr {
    pub mv_count: usize,
    pub from: char,
    pub to: char,
}

#[derive(Debug)]
pub struct Input {
    pub stacks: HashMap<char, Vec<char>>,
    pub instrs: Vec<Instr>,
}

pub struct Day5 {}
impl Day5 {
    fn parse_stack(inp: &[String]) -> HashMap<char, Vec<char>> {
        // inp.chun
        let mut linewise_hmap = inp
            .iter()
            .map(|line| {
                line.chars()
                    .collect::<Vec<char>>()
                    .as_slice()
                    .chunks(4 /*"[_] " => 4 characters*/)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x.get(1).unwrap() == ' ' {
                            None
                        } else {
                            Some((i, x.get(1).unwrap().to_owned()))
                        }
                    })
                    .collect::<HashMap<usize, char>>()
            })
            .collect::<Vec<HashMap<usize, char>>>();

        let mut indices = linewise_hmap
            .pop()
            .unwrap()
            .values()
            .map(|x| x.to_owned())
            .collect::<Vec<char>>();
        indices.sort();
        linewise_hmap
            .iter()
            .rev()
            .fold(HashMap::new(), |mut acc, e| {
                for (i, c) in indices.iter().enumerate() {
                    if let Some(x) = e.get(&i) {
                        acc.entry(c.to_owned()).or_default().push(x.to_owned());
                    }
                }
                acc
            })
    }
    fn parse_instrs(inp: &[String]) -> Vec<Instr> {
        inp.iter()
            .map(|x| {
                let out = x.chars().fold((vec![], false), |mut acc, e| {
                    if e.is_ascii_digit() {
                        if acc.1 {
                            *acc.0.last_mut().unwrap() *= 10;
                            *acc.0.last_mut().unwrap() += e.to_digit(10).unwrap() as usize;
                        } else {
                            acc.0.push(e.to_digit(10).unwrap() as usize);
                        }
                        (acc.0, true)
                    } else {
                        (acc.0, false)
                    }
                });
                Instr {
                    mv_count: *out.0.first().unwrap(),
                    from: (*out.0.get(1).unwrap() as u8 + b'0') as char,
                    to: (*out.0.get(2).unwrap() as u8 + b'0') as char,
                }
            })
            .collect::<Vec<Instr>>()
    }
    fn parse() -> Input {
        let buf = get_buffer("input/day5.txt");
        let _two_parts = buf
            .lines()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>()
            .split(|x| x.is_empty())
            .map(|x| x.to_owned())
            .collect::<Vec<Vec<String>>>();
        Input {
            stacks: Day5::parse_stack(_two_parts.first().unwrap()),
            instrs: Day5::parse_instrs(_two_parts.last().unwrap()),
        }
    }

    pub fn common<F: Fn(Vec<char>) -> Vec<char>>(f: F) -> String {
        let inp = Day5::parse();
        let mut stacks = inp.stacks.clone();

        for instr in inp.instrs {
            let temp = (0..(instr.mv_count))
                .map(|_| stacks.get_mut(&instr.from).unwrap().pop().unwrap())
                .collect::<Vec<char>>();
            f(temp)
                .into_iter()
                .for_each(|x| stacks.get_mut(&instr.to).unwrap().push(x));
        }
        let mut temp = stacks.into_iter().collect::<Vec<(char, Vec<char>)>>();
        temp.sort();
        temp.iter()
            .map(|x| x.1.last().map(|x| x.to_string()).unwrap())
            .collect::<String>()
    }
    pub fn part_1() -> String {
        Day5::common(|x| x)
    }

    pub fn part_2() -> String {
        Day5::common(|x| x.into_iter().rev().collect())
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day5;
        println!("{:?}", Day5::part_1());
        println!("{:?}", Day5::part_2());
    }
}
