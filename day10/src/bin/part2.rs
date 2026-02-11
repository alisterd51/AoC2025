use std::io::{self, Read};

use day10::{Machine, parse_machines, solve_part_2};

fn get_input() -> Vec<Machine> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_machines(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_2(&input);
    println!("{result}");
}
