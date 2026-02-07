#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

#[derive(Copy, Clone)]
pub enum Cell {
    Paper,
    Empty,
    Other,
}

const fn parse_cell(c: char) -> Cell {
    match c {
        '@' => Cell::Paper,
        '.' => Cell::Empty,
        _ => Cell::Other,
    }
}

pub fn parse_grid(str: &str) -> Vec<Vec<Cell>> {
    let mut grid = vec![];

    for line in str.split_whitespace() {
        let row = line.chars().map(parse_cell).collect();
        grid.push(row);
    }
    grid
}

fn is_accessible(grid: &[Vec<Cell>], y: usize, x: usize) -> bool {
    let mut adjacent_papers = 0;
    for derive in &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let y_len = grid.len();
        let x_len = grid[y].len();
        let y = y as i32 + derive.0;
        let x = x as i32 + derive.1;
        if y >= 0
            && y < y_len as i32
            && x >= 0
            && x < x_len as i32
            && matches!(grid[y as usize][x as usize], Cell::Paper)
        {
            adjacent_papers += 1;
        }
        if adjacent_papers >= 4 {
            return false;
        }
    }
    true
}

#[must_use]
pub fn solve_part_1(grid: &[Vec<Cell>]) -> u64 {
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if matches!(grid[y][x], Cell::Paper) && is_accessible(grid, y, x) {
                result += 1;
            }
        }
    }

    result
}

#[must_use]
pub fn solve_part_2(grid: &mut [Vec<Cell>]) -> u64 {
    let mut result = 0;
    let mut updated = true;
    while updated {
        updated = false;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if matches!(grid[y][x], Cell::Paper) && is_accessible(grid, y, x) {
                    result += 1;
                    grid[y][x] = Cell::Empty;
                    updated = true;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let input = parse_grid(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn example_solve_part_2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let mut input = parse_grid(input);
        let result = solve_part_2(&mut input);
        assert_eq!(result, 43);
    }
}
