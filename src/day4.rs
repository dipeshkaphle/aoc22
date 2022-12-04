use std::io::BufRead;

use crate::get_buffer;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub l: usize,
    pub r: usize,
}

impl Range {
    pub fn contains(&self, other_range: &Range) -> bool {
        self.l <= other_range.l && self.r >= other_range.r
    }
    pub fn overlaps(&self, other_range: &Range) -> bool {
        let smaller = self.min(other_range);
        let larger = self.max(other_range);
        larger.l >= smaller.l && larger.l <= smaller.r
    }
}

pub struct Day4 {}
impl Day4 {
    fn parse() -> Vec<(Range, Range)> {
        let buf = get_buffer("input/day4.txt");
        buf.lines()
            .map(|x| x.unwrap())
            .map(|x| {
                x.split(',')
                    .map(|y| {
                        y.to_owned()
                            .split('-')
                            .map(|z| z.to_owned())
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<Vec<String>>>()
            })
            .map(|x| {
                return (
                    Range {
                        l: x.get(0).unwrap().get(0).unwrap().parse::<usize>().unwrap(),
                        r: x.get(0).unwrap().get(1).unwrap().parse::<usize>().unwrap(),
                    },
                    Range {
                        l: x.get(1).unwrap().get(0).unwrap().parse::<usize>().unwrap(),
                        r: x.get(1).unwrap().get(1).unwrap().parse::<usize>().unwrap(),
                    },
                );
            })
            .collect::<Vec<(Range, Range)>>()
    }

    pub fn part_1() -> usize {
        let ranges = Day4::parse();
        ranges
            .iter()
            .filter(|(r1, r2)| r1.contains(r2) || r2.contains(r1))
            .count()
    }

    pub fn part_2() -> usize {
        let ranges = Day4::parse();
        ranges.iter().filter(|(r1, r2)| r1.overlaps(r2)).count()
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day4;
        println!("{:?}", Day4::part_1());
        println!("{:?}", Day4::part_2());
    }
}
