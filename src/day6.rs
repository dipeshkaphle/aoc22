use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

use crate::get_buffer;

pub struct Day6 {}
impl Day6 {
    fn parse() -> Vec<char> {
        let buf = get_buffer("input/day6.txt");
        buf.lines()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>()
            .get(0)
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
    }

    pub fn common(sz: usize) -> usize {
        let inp = Day6::parse();
        let mut dq = VecDeque::new();
        inp.iter()
            .position(|x| {
                dq.push_back(x);
                if dq.len() < sz {
                    return false;
                } else if dq.len() > sz {
                    dq.pop_front();
                }
                dq.iter()
                    .map(|x| x.to_owned())
                    .collect::<HashSet<&char>>()
                    .len()
                    == sz
            })
            .unwrap()
            + 1
    }
    pub fn part_1() -> usize {
        Day6::common(4)
    }

    pub fn part_2() -> usize {
        Day6::common(14)
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day6;
        println!("{:?}", Day6::part_1());
        println!("{:?}", Day6::part_2());
    }
}
