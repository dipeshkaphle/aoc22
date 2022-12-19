use std::{collections::BTreeMap, io::BufRead, ops::Bound};

use crate::get_buffer;

#[derive(Debug)]
pub enum Instr {
    Addx(i64),
    NoOp,
}

#[derive(Debug)]
pub struct Machine {
    pub instrs: Vec<Instr>,
    pub ip: usize,
    pub time: usize,
    pub reg: i64,
    pub reg_history: BTreeMap<i64, i64>,
}
impl Machine {
    pub fn has_next(&self) -> bool {
        self.ip < self.instrs.len()
    }
    pub fn execute_next(&mut self) {
        match self.instrs.get(self.ip).unwrap() {
            Instr::Addx(x) => {
                self.reg += x;
                self.time += 2;
                self.reg_history.insert(self.time as i64, self.reg);
            }
            Instr::NoOp => {
                self.time += 1;
            }
        }
        self.ip += 1;
    }
}

pub struct Day10 {}
impl Day10 {
    fn parse() -> Vec<Instr> {
        let buf = get_buffer("input/day10.txt");
        buf.lines()
            .map(|x| {
                let x = x.unwrap();
                let s = x.split_whitespace().collect::<Vec<&str>>();
                if x.starts_with("addx") {
                    Instr::Addx(s.get(1).unwrap().parse::<i64>().unwrap())
                } else {
                    Instr::NoOp
                }
            })
            .collect()
    }

    pub fn part_1() -> i64 {
        let instrs = Day10::parse();
        let mut state = Machine {
            instrs,
            ip: 0,
            time: 0,
            reg: 1,
            reg_history: {
                let mut bm = BTreeMap::new();
                bm.insert(0, 1);
                bm
            },
        };
        while state.has_next() {
            state.execute_next();
        }
        [20, 60, 100, 140, 180, 220]
            .iter()
            .map(|x| {
                let res = state.reg_history.range(..(*x)).last().unwrap();
                x * *res.1
                // 1
            })
            .sum()
    }

    pub fn part_2() {
        let instrs = Day10::parse();
        let mut state = Machine {
            instrs,
            ip: 0,
            time: 0,
            reg: 1,
            reg_history: {
                let mut bm = BTreeMap::new();
                bm.insert(-1, 1);
                bm
            },
        };
        while state.has_next() {
            state.execute_next();
        }
        println!("{:?}", state.reg_history);
        let mut grid = vec![vec!['.'; 40]; 6];
        for i in 0..240 {
            let row_no = i / 40;
            let col_no = i % 40;

            let before = state.reg_history.range(..=i).last();
            if (col_no as i64).abs_diff(*before.unwrap().1) <= 1 {
                *grid
                    .get_mut(row_no as usize)
                    .unwrap()
                    .get_mut(col_no as usize)
                    .unwrap() = '#';
            }
        }
        for x in grid {
            let y = x
                .iter()
                .map(|y| y.to_string())
                .collect::<Vec<String>>()
                .join("");
            println!("{}", y);
        }
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day10;
        println!("{:?}", Day10::part_1());
        println!("{:?}", Day10::part_2());
    }
}
