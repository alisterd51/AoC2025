use std::io::{self, Read};

use day8::{Coord, parse_coords, solve_part_1};

fn get_input() -> Vec<Coord> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_coords(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_1(&input, 1000);
    println!("{result}");
}
