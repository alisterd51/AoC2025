pub struct Coord {
    x: u64,
    y: u64,
}

#[must_use]
pub fn parse_coords(input: &str) -> Vec<Coord> {
    let mut coords = vec![];

    for line in input.split_whitespace() {
        if let Some((x, y)) = line.split_once(',')
            && let Ok(x) = x.parse::<u64>()
            && let Ok(y) = y.parse::<u64>()
        {
            coords.push(Coord { x, y });
        }
    }

    coords
}

#[must_use]
pub fn solve_part_1(coords: &[Coord]) -> u64 {
    let mut result = 0;
    for (index, coord_a) in coords.iter().enumerate().take(coords.len() - 1) {
        for coord_b in coords.iter().skip(index + 1) {
            let area = (coord_a.x.abs_diff(coord_b.x) + 1) * (coord_a.y.abs_diff(coord_b.y) + 1);
            if result < area {
                result = area;
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
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let input = parse_coords(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 50);
    }
}
