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

#[allow(clippy::suspicious_operation_groupings)]
#[must_use]
pub fn solve_part_2(data: &Data) -> u64 {
    let mut result = 0;
    let mut new_ranges: Vec<(u64, u64)> = vec![];
    for (index, range) in data.ranges.iter().enumerate() {
        let mut new_range = *range;
        for other_range in &data.ranges[(index + 1)..] {
            if other_range.0 <= new_range.0 && new_range.0 <= other_range.1 {
                new_range.0 = other_range.1 + 1;
            }
            if other_range.0 <= new_range.1 && new_range.1 <= other_range.1 {
                new_range.1 = other_range.0 - 1;
            }
        }
        for other_range in &new_ranges {
            if other_range.0 <= new_range.0 && new_range.0 <= other_range.1 {
                new_range.0 = other_range.1 + 1;
            }
            if other_range.0 <= new_range.1 && new_range.1 <= other_range.1 {
                new_range.1 = other_range.0 - 1;
            }
        }
        if new_range.0 <= new_range.1 {
            new_ranges.push(new_range);
        }
    }
    for range in &new_ranges {
        result += range.1 - range.0 + 1;
    }

    result
}

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
    fn example_solve_part_2() {
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
        let result = solve_part_2(&input);
        assert_eq!(result, 14);
    }

    #[test]
    fn custom_solve_part_2() {
        let input = "110-120
100-200
";
        let input = parse_data(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 101);
        let input = "100-200
110-120
";
        let input = parse_data(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 101);
    }
}
