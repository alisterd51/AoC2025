use std::io::{self, Read};

use day7::{Item, parse_grid, solve_part_1};

fn get_input() -> Vec<Vec<Item>> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_grid(&buf)
}

fn main() {
    let mut input = get_input();
    let result = solve_part_1(&mut input);
    println!("{result}");
}
