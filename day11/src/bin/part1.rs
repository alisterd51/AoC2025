use std::io::{self, Read};

use day11::{Graph, parse_graph, solve_part_1};

fn get_input() -> Graph {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_graph(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_1(&input);
    println!("{result}");
}
