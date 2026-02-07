use std::io::{self, Read};

use day5::{Data, parse_data, solve_part_1};

fn get_input() -> Data {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_data(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_1(&input);
    println!("{result}");
}
