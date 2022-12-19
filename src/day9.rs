use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    io::BufRead,
};

use crate::get_buffer;

#[derive(Debug)]
pub enum Movement {
    L(usize),
    R(usize),
    D(usize),
    U(usize),
}

#[derive(Debug, Default, Clone)]
pub struct SnakeGrid {
    vs: BTreeSet<(i64, i64)>,
    head: (i64, i64),
    tail: (i64, i64),
}
impl SnakeGrid {
    pub fn peek_next_head_move(&self, movement: &Movement) -> (i64, i64) {
        match movement {
            Movement::L(_) => (self.head.0, self.head.1 - 1),
            Movement::R(_) => (self.head.0, self.head.1 + 1),
            Movement::U(_) => (self.head.0 + 1, self.head.1),
            Movement::D(_) => (self.head.0 - 1, self.head.1),
        }
    }
    pub fn is_within_one_distance(hp: (i64, i64), tp: (i64, i64)) -> bool {
        hp.0.abs_diff(tp.0) <= 1 && hp.1.abs_diff(tp.1) <= 1
    }
    pub fn get_dx_dy(cur: &(i64, i64), prev: &(i64, i64)) -> (i64, i64) {
        (cur.0 - prev.0, cur.1 - prev.1)
    }
    pub fn peek_next_tail_move(hd: &(i64, i64), tl: &(i64, i64)) -> (i64, i64) {
        // println!("======> {:?} to {:?}", tl, hd);
        if hd.0 == tl.0 {
            if hd.1 > tl.1 {
                (0, 1)
            } else {
                (0, -1)
            }
        } else if hd.1 == tl.1 {
            if hd.0 > tl.0 {
                (1, 0)
            } else {
                (-1, 0)
            }
        } else {
            let (dx, dy) = SnakeGrid::get_dx_dy(hd, tl);
            (dx / dx.abs(), dy / dy.abs())
        }
    }
}

fn printer(pos: &Vec<(i64, i64)>) {
    let x = pos.iter().map(|x| x.0).max().unwrap() + 1;
    let y = pos.iter().map(|x| x.1).max().unwrap() + 1;
    let xmin = pos.iter().map(|x| x.0).min().unwrap().abs();
    let ymin = pos.iter().map(|x| x.1).min().unwrap().abs();
    let mut v: Vec<Vec<char>> =
        vec![vec!['.'; y as usize + ymin as usize]; x as usize + xmin as usize];
    for (index, (i, j)) in pos.iter().enumerate() {
        *v.get_mut(*i as usize + xmin as usize)
            .unwrap()
            .get_mut(*j as usize + ymin as usize)
            .unwrap() = char::from_digit(index as u32, 10).unwrap();
    }
    for r in v.iter() {
        println!("{:?}", r);
    }
}

pub struct Day9 {}
impl Day9 {
    fn parse() -> Vec<Movement> {
        let buf = get_buffer("input/day9.txt");
        buf.lines()
            .map(|x| {
                let x = x.unwrap();
                let tokens = x.split_whitespace().collect::<Vec<&str>>();
                let num = tokens.get(1).unwrap().parse::<usize>().unwrap();
                return match *tokens.first().unwrap() {
                    "L" => Movement::L(num),
                    "R" => Movement::R(num),
                    "U" => Movement::U(num),
                    "D" => Movement::D(num),
                    _ => unreachable!(),
                };
            })
            .collect()
    }

    pub fn part_1() -> usize {
        let movements = Day9::parse();
        let mut grid = SnakeGrid::default();
        grid.vs.insert((0, 0));
        let new_grid = movements.iter().fold(grid, |mut acc, e| {
            match e {
                Movement::L(delta)
                | Movement::R(delta)
                | Movement::U(delta)
                | Movement::D(delta) => {
                    (0..(*delta)).for_each(|_| {
                        let new_pos = acc.peek_next_head_move(e);
                        if !SnakeGrid::is_within_one_distance(new_pos, acc.tail) {
                            let (dx, dy) = SnakeGrid::peek_next_tail_move(&new_pos, &acc.tail);
                            acc.tail = (acc.tail.0 + dx, acc.tail.1 + dy);
                            acc.vs.insert(acc.tail);
                        }
                        acc.head = new_pos;
                    });
                }
            }
            acc
        });
        new_grid.vs.len()
    }

    pub fn part_2() -> usize {
        let movements = Day9::parse();
        let mut grids = vec![SnakeGrid::default(); 9];
        grids.iter_mut().for_each(|g| {
            g.vs.insert((0, 0));
        });
        let new_grids = movements.iter().fold(grids, |mut acc, e| {
            match e {
                Movement::L(delta)
                | Movement::R(delta)
                | Movement::U(delta)
                | Movement::D(delta) => {
                    (0..(*delta)).for_each(|_| {
                        let mut new_pos = acc.first().unwrap().peek_next_head_move(e);
                        let mut changed = true;
                        for (_i, grid) in acc.iter_mut().enumerate() {
                            if !changed {
                                break;
                            }
                            grid.head = new_pos;
                            changed = false;
                            if !SnakeGrid::is_within_one_distance(grid.head, grid.tail) {
                                changed = true;
                                let (dx, dy) =
                                    SnakeGrid::peek_next_tail_move(&grid.head, &grid.tail);
                                grid.tail = (grid.tail.0 + dx, grid.tail.1 + dy);
                                new_pos = grid.tail;
                                grid.vs.insert(grid.tail);
                            }
                        }
                    });
                }
            }
            acc
        });
        new_grids.last().unwrap().vs.len()
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day9;
        println!("{:?}", Day9::part_1());
        println!("{:?}", Day9::part_2());
    }
}
