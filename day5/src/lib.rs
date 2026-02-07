enum ParsedLine {
    Range((u64, u64)),
    Id(u64),
}

#[derive(Debug)]
pub struct Data {
    ranges: Vec<(u64, u64)>,
    ids: Vec<u64>,
}

fn parse_line(line: &str) -> Option<ParsedLine> {
    if let Some(range) = line.split_once('-')
        && let Ok(a) = range.0.parse::<u64>()
        && let Ok(b) = range.1.parse::<u64>()
    {
        Some(ParsedLine::Range((a, b)))
    } else if let Ok(id) = line.parse::<u64>() {
        Some(ParsedLine::Id(id))
    } else {
        None
    }
}

#[must_use]
pub fn parse_data(input: &str) -> Data {
    let mut ranges = vec![];
    let mut ids = vec![];

    for line in input.split_whitespace() {
        match parse_line(line) {
            Some(ParsedLine::Range(range)) => ranges.push(range),
            Some(ParsedLine::Id(id)) => ids.push(id),
            None => {}
        }
    }
    Data { ranges, ids }
}

#[must_use]
pub fn solve_part_1(data: &Data) -> u64 {
    let mut result = 0;
    for id in &data.ids {
        for range in &data.ranges {
            if range.0 <= *id && *id <= range.1 {
                result += 1;
                break;
            }
        }
    }

    result
}

// #[allow(unused_mut)]
// #[must_use]
// pub fn solve_part_2(_grid: &Data) -> u64 {
//     let mut result = 0;

//     result
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
        let input = parse_data(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_solve_part_2() {}
}
