fn parse_range(range: &str) -> Option<(u64, u64)> {
    if let Some(range) = range.split_once('-')
        && let Ok(a) = range.0.parse::<u64>()
        && let Ok(b) = range.1.parse::<u64>()
    {
        Some((a, b))
    } else {
        None
    }
}

#[must_use]
pub fn solve_part_1(input: Vec<&str>) -> u64 {
    let mut result = 0;
    for range in input {
        if let Some(range) = parse_range(range) {
            for id in range.0..=range.1 {
                let id_str = id.to_string();
                let id_len = id_str.len();
                if id_len % 2 == 0 {
                    let a = &id_str[..(id_len / 2)];
                    let b = &id_str[(id_len / 2)..];
                    if a == b {
                        result += id;
                    }
                }
            }
        }
    }

    result
}

fn is_magic(s: &str, steps: usize) -> bool {
    let mut value = None;
    for part in (0..s.len())
        .step_by(steps)
        .map(|i| &s[i..(i + steps).min(s.len())])
    {
        match value {
            Some(value) => {
                if value != part {
                    return false;
                }
            }
            None => value = Some(part),
        }
    }
    true
}

#[must_use]
pub fn solve_part_2(input: Vec<&str>) -> u64 {
    let mut result = 0;
    for range in input {
        if let Some(range) = parse_range(range) {
            for id in range.0..=range.1 {
                let id_str = id.to_string();
                let id_len = id_str.len();
                for steps in 1..id_len {
                    if id_len % steps == 0 && is_magic(&id_str, steps) {
                        result += id;
                        break;
                    }
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
        let input = vec!["11-22"];
        let result = solve_part_1(input);
        assert_eq!(result, 11 + 22);
        let input = vec!["11-22", "95-115"];
        let result = solve_part_1(input);
        assert_eq!(result, 11 + 22 + 99);
        let input = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ];
        let result = solve_part_1(input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn example_solve_part_2() {
        let input = vec!["11-22"];
        let result = solve_part_2(input);
        assert_eq!(result, 11 + 22);
        let input = vec!["11-22", "95-115"];
        let result = solve_part_2(input);
        assert_eq!(result, 11 + 22 + 99 + 111);
        let input = vec!["95-115"];
        let result = solve_part_2(input);
        assert_eq!(result, 99 + 111);
        let input = vec!["998-1012"];
        let result = solve_part_2(input);
        assert_eq!(result, 999 + 1010);
        let input = vec!["1188511880-1188511890"];
        let result = solve_part_2(input);
        assert_eq!(result, 1188511885);
        let input = vec!["222220-222224"];
        let result = solve_part_2(input);
        assert_eq!(result, 222222);
        let input = vec!["1698522-1698528"];
        let result = solve_part_2(input);
        assert_eq!(result, 0);
        let input = vec!["446443-446449"];
        let result = solve_part_2(input);
        assert_eq!(result, 446446);
        let input = vec!["565653-565659"];
        let result = solve_part_2(input);
        assert_eq!(result, 565656);
        let input = vec!["824824821-824824827"];
        let result = solve_part_2(input);
        assert_eq!(result, 824824824);
        let input = vec!["2121212118-2121212124"];
        let result = solve_part_2(input);
        assert_eq!(result, 2121212121);
        let input = vec![
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ];
        let result = solve_part_2(input);
        assert_eq!(result, 4174379265);
    }
}
