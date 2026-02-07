#[derive(Clone, Copy)]
pub enum Item {
    Source,
    Empty,
    Splitter,
    Tachyon,
    Other,
}

const fn parse_item(c: char) -> Item {
    match c {
        'S' => Item::Source,
        '.' => Item::Empty,
        '^' => Item::Splitter,
        '|' => Item::Tachyon,
        _ => Item::Other,
    }
}

pub fn parse_grid(str: &str) -> Vec<Vec<Item>> {
    let mut grid = vec![];

    for line in str.split_whitespace() {
        let row = line.chars().map(parse_item).collect();
        grid.push(row);
    }
    grid
}

#[allow(unused_mut)]
#[must_use]
pub fn solve_part_1(grid: &mut [Vec<Item>]) -> u64 {
    let mut result = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x] = match &grid[y][x] {
                Item::Source => Item::Tachyon,
                Item::Empty => {
                    if y > 0
                        && (matches!(grid[y - 1][x], Item::Tachyon)
                            || (x > 0
                                && matches!(grid[y][x - 1], Item::Splitter)
                                && matches!(grid[y - 1][x - 1], Item::Tachyon))
                            || (x < grid[y].len() - 1
                                && matches!(grid[y][x + 1], Item::Splitter)
                                && matches!(grid[y - 1][x + 1], Item::Tachyon)))
                    {
                        Item::Tachyon
                    } else {
                        Item::Empty
                    }
                }
                Item::Splitter => {
                    if y > 0 && matches!(grid[y - 1][x], Item::Tachyon) {
                        result += 1;
                    }
                    Item::Splitter
                }
                item => *item,
            };
        }
    }

    result
}

// #[allow(unused_variables)]
// #[allow(unused_mut)]
// #[must_use]
// pub fn solve_part_2(grid: &[Vec<Item>]) -> u64 {
//     let mut result = 0;

//     result
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let mut input = parse_grid(input);
        let result = solve_part_1(&mut input);
        assert_eq!(result, 21);
    }

    #[test]
    fn example_solve_part_2() {}
}
