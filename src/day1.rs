use std::io::BufRead;

use crate::get_buffer;

pub struct Elf {
    pub calories: usize,
    pub elf_no: usize,
}

pub struct Day1 {}
impl Day1 {
    fn parse() -> Vec<Elf> {
        let buf = get_buffer("input/day1.txt");
        let mut elfs = vec![Elf {
            calories: 0,
            elf_no: 1,
        }];
        buf.lines().map(|x| x.unwrap()).for_each(|x| {
            if x.is_empty() {
                elfs.push(Elf {
                    calories: 0,
                    elf_no: elfs.len() + 1,
                });
            } else {
                let last_elf = elfs.last_mut().unwrap();
                let calory = x.parse::<usize>().unwrap();
                last_elf.calories += calory;
            }
        });
        elfs
    }

    fn sort_elfs_by_calories() -> Vec<Elf> {
        let mut elfs = Day1::parse();
        elfs.sort_by(|x, y| x.calories.cmp(&y.calories));
        elfs
    }
    pub fn part_1() -> usize {
        Day1::sort_elfs_by_calories().last().unwrap().calories
    }

    pub fn part_2() -> usize {
        let sorted_elf = Day1::sort_elfs_by_calories();
        sorted_elf.iter().rev().take(3).map(|x| x.calories).sum()
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day1;
        println!("{:?}", Day1::part_1());
        println!("{:?}", Day1::part_2());
    }
}
