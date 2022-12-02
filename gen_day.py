import sys
day_no = int(sys.argv[1])

template_code = f"""
use std::io::BufRead;

use crate::get_buffer;

pub struct Day{day_no} {{}}
impl Day{day_no} {{
    fn parse() {{
        let buf = get_buffer("input/day{day_no}.txt");
    }}

    pub fn part_1() {{}}

    pub fn part_2() {{}}
}}

mod tests {{
    #[test]
    fn it_works() {{
        use super::Day{day_no};
        println!("{{:?}}", Day{day_no}::part_1());
        println!("{{:?}}", Day{day_no}::part_2());
    }}
}}
"""
file_name = f"src/day{day_no}.rs"

open(file_name,"a").write(template_code)
