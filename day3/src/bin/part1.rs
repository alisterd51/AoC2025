use std::io::{self, Read};

use day3::solve_part_1;

fn parse_str(str: &str) -> Vec<String> {
    let mut parsed_str = vec![];

    for word in str.split_whitespace() {
        parsed_str.push(word.to_string());
    }
    parsed_str
}

fn get_input() -> Vec<String> {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);

    parse_str(&buf)
}

fn main() {
    let input = get_input();
    let result = solve_part_1(input.iter().map(std::string::String::as_str).collect());
    println!("{result}");
}
