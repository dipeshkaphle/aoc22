use std::{io::BufRead, process::Output};

use crate::get_buffer;

#[derive(Debug)]
pub enum Line {
    Command(String),
    Output(Vec<String>),
}

#[derive(Debug)]
pub enum Cmd {
    Ls,
    Cd(String),
}

type ChildrenIndices = Vec<usize>;

#[derive(Debug, Clone)]
pub enum LsOut {
    Dir(String),
    File(String, usize),
}

#[derive(Debug)]
pub struct DirTree {
    pub entries: Vec<(LsOut, ChildrenIndices)>,
    pub parents: Vec<usize>,
    pub cur: usize,
}

impl DirTree {
    pub fn fill_children_indices(&mut self, ls_out: &Vec<LsOut>) {
        self.entries.get_mut(self.cur).unwrap().1 = vec![];
        for x in ls_out {
            let new_entry_index = self.entries.len();
            self.entries.push((x.clone(), vec![]));
            self.parents.push(self.cur);
            self.entries
                .get_mut(self.cur)
                .unwrap()
                .1
                .push(new_entry_index);
        }
    }
    pub fn handle_cd(&mut self, dir: &str) {
        if dir == ".." {
            self.cur = *self.parents.get(self.cur).unwrap();
        } else {
            for i in self.entries.get(self.cur).unwrap().1.iter() {
                let e = self.entries.get(*i).unwrap();
                match &e.0 {
                    LsOut::Dir(x) => {
                        if x == dir {
                            self.cur = *i;
                            break;
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }
    pub fn get_all_dir_sizes(&mut self, sizes: &mut Vec<usize>) -> usize {
        let original_id = self.cur;
        let mut tot_size = 0;
        for i in self.entries.get(self.cur).unwrap().1.clone() {
            // self.cur = i;
            match &self.entries.get(i).unwrap().0 {
                LsOut::Dir(_) => {
                    self.cur = i;
                    tot_size += self.get_all_dir_sizes(sizes);
                }
                LsOut::File(_, sz) => {
                    tot_size += sz;
                }
            }
            self.cur = original_id;
        }
        sizes.push(tot_size);

        tot_size
    }
}

#[derive(Debug)]
pub struct State {
    dir_tree: DirTree,
}
impl State {
    pub fn update_state(&mut self, cmd: &Cmd, ls_out: Option<&Vec<LsOut>>) {
        match cmd {
            Cmd::Ls => {
                assert!(ls_out.is_some());
                let ls_out = ls_out.unwrap();
                self.dir_tree.fill_children_indices(ls_out);
            }
            Cmd::Cd(dir) => {
                self.dir_tree.handle_cd(dir);
            }
        }
    }
}

pub struct Day7 {}
impl Day7 {
    fn parse() -> Vec<Line> {
        let buf = get_buffer("input/day7.txt");
        buf.lines().map(|x| x.unwrap()).fold(vec![], |mut acc, ln| {
            if ln.starts_with('$') {
                acc.push(Line::Command(ln[2..].to_owned()));
                acc.push(Line::Output(vec![]));
            } else {
                let last = acc.last_mut().expect("Vec can't be empty here");
                match last {
                    Line::Output(v) => {
                        v.push(ln);
                    }
                    _ => {
                        panic!("This should never happen");
                    }
                }
            }
            acc
        })
    }

    pub fn parse_cmd(s: &str) -> Cmd {
        if s.starts_with("cd") {
            Cmd::Cd(s[3..].to_owned())
        } else if s.starts_with("ls") {
            Cmd::Ls
        } else {
            unimplemented!()
        }
    }

    pub fn parse_ls(s: &str) -> LsOut {
        if s.starts_with("dir") {
            LsOut::Dir(s[4..].to_owned())
        } else {
            let split = s.split(' ').collect::<Vec<&str>>();
            LsOut::File(
                split.get(1).unwrap().to_string(),
                split.first().unwrap().parse::<usize>().unwrap(),
            )
        }
    }

    pub fn common() -> Vec<usize> {
        let inp = Day7::parse();
        let cmd_and_out = inp
            .chunks(2)
            .map(|x| (x.get(0).unwrap(), x.get(1).unwrap()));
        let mut st = State {
            // pwd: "/".to_string(),
            dir_tree: DirTree {
                entries: vec![(LsOut::Dir("/".to_string()), vec![])],
                parents: vec![0],
                cur: 0,
            },
        };
        cmd_and_out.for_each(|(c, o)| {
            // println!("{:?}", (c, o));
            match (c, o) {
                (Line::Command(c), Line::Output(o)) => {
                    let cmd = Day7::parse_cmd(c);
                    match &cmd {
                        Cmd::Cd(_) => {
                            st.update_state(&cmd, None);
                        }
                        Cmd::Ls => {
                            let ls_out =
                                o.iter().map(|x| Day7::parse_ls(x)).collect::<Vec<LsOut>>();
                            st.update_state(&cmd, Some(&ls_out));
                        }
                    }
                }
                _ => panic!("Not possible because of the way it's parsed"),
            }
        });
        st.dir_tree.cur = 0;
        let mut sizes = vec![];
        st.dir_tree.get_all_dir_sizes(&mut sizes);
        sizes
    }
    pub fn part_1() -> usize {
        let sizes = Day7::common();
        sizes.into_iter().filter(|x| *x <= 100000usize).sum()
    }

    pub fn part_2() -> usize {
        let mut sizes = Day7::common();
        sizes.sort();
        let total = 70000000usize;
        let used = *sizes.last().unwrap();
        let remaining = total - used;
        let to_be_freed = 30000000usize - remaining;
        sizes.into_iter().find(|x| *x >= to_be_freed).unwrap()
    }
}

mod tests {
    #[test]
    fn it_works() {
        use super::Day7;
        // println!("{:?}", Day7::parse());
        println!("{:?}", Day7::part_1());
        println!("{:?}", Day7::part_2());
    }
}
