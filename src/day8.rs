use std::io::BufRead;

use crate::get_buffer;

pub struct Day8 {}
impl Day8 {
    fn parse() -> Vec<Vec<i8>> {
        let buf = get_buffer("input/day8.txt");
        buf.lines()
            .map(|x| {
                x.unwrap()
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as i8)
                    .collect::<Vec<i8>>()
            })
            .collect()
    }

    pub fn part_1() -> usize{
        let grid = Day8::parse();
        let rows = grid.len();
        let mut cnt =0;
        for i in 0..rows {
            let slice = &grid.get(i).unwrap()[..];
            let cols = grid.get(i).unwrap().len();
            for j in 0..cols {
                let v = slice[j];
                let left_max = slice[..j].iter().max().unwrap_or(&-1);
                let right_max = slice[(j + 1)..].iter().max().unwrap_or(&-1);
                let top_max = grid[..i]
                    .iter()
                    .map(|x| x[..][j])
                    .max()
                    .unwrap_or(-1);
                let bottom_max= grid[(i+1)..]
                    .iter()
                    .map(|x| x[..][j])
                    .max()
                    .unwrap_or(-1);
                if v>*left_max || v>*right_max || v > top_max || v>bottom_max {
                    cnt+=1;
                }
            }
        }
        cnt
    }

    pub fn part_2() -> usize {
        let grid = Day8::parse();
        let rows = grid.len();
        let mut vals = vec![];
        for i in 0..rows{
            let slice = &grid.get(i).unwrap()[..];
            let cols = grid.get(i).unwrap().len();
            for j in 0..cols{
                let h = slice[j];
                let left = &slice[..j];
                let right = &slice[(j+1)..];
                let top = &grid[..i];
                let bottom = &grid[(i+1)..];
                let lv = left.iter().rev().position(|x|  *x>= h  ).map(|x| x+1).unwrap_or(left.len());
                let rv = right.iter().position(|x| *x>=h).map(|x| x+1).unwrap_or(right.len());
                let tv = top.iter().map(|x| x[..][j]).rev().position(|x| x>=h).map(|x| x+1).unwrap_or(top.len());
                let bv = bottom.iter().map(|x| x[..][j]).position(|x| x>=h).map(|x| x+1).unwrap_or(bottom.len());
                let beauty = lv*rv*tv*bv;
                // println!("[{},{}]=> {}", i, j, beauty);
                vals.push(beauty);
            }
        }
        *vals.iter().max().unwrap()
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day8;
        println!("{:?}", Day8::part_1());
        println!("{:?}", Day8::part_2());
    }
}
