#[must_use]
pub fn solve_part_1(input: Vec<&str>) -> u64 {
    let mut result = 0;

    for bank in input {
        let mut index_max = 0;
        let mut batterie_max = 0;
        for (index, batterie) in bank
            .chars()
            .filter_map(|c| c.to_digit(10).map(u64::from))
            .enumerate()
        {
            if batterie_max < batterie {
                index_max = index;
                batterie_max = batterie;
            }
        }
        if index_max < bank.len() - 1 {
            result += 10 * batterie_max;
            let bank = &bank[(index_max + 1)..];
            let mut batterie_max = 0;
            for batterie in bank.chars().filter_map(|c| c.to_digit(10).map(u64::from)) {
                if batterie_max < batterie {
                    batterie_max = batterie;
                }
            }
            result += batterie_max;
        } else {
            result += batterie_max;
            let bank = &bank[..index_max];
            let mut batterie_max = 0;
            for batterie in bank.chars().filter_map(|c| c.to_digit(10).map(u64::from)) {
                if batterie_max < batterie {
                    batterie_max = batterie;
                }
            }
            result += 10 * batterie_max;
        }
    }
    result
}

#[must_use]
pub fn solve_part_2(_input: Vec<&str>) -> u64 {
    // let mut result = 0;

    // result
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = vec!["987654321111111"];
        let result = solve_part_1(input);
        assert_eq!(result, 98);
        let input = vec!["811111111111119"];
        let result = solve_part_1(input);
        assert_eq!(result, 89);
        let input = vec!["234234234234278"];
        let result = solve_part_1(input);
        assert_eq!(result, 78);
        let input = vec!["818181911112111"];
        let result = solve_part_1(input);
        assert_eq!(result, 92);
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let result = solve_part_1(input);
        assert_eq!(result, 357);
    }

    #[test]
    fn custom_solve_part_1() {
        let input = vec!["111111119"];
        let result = solve_part_1(input);
        assert_eq!(result, 19);
    }

    #[test]
    fn example_solve_part_2() {
        let input = vec![];
        let result = solve_part_2(input);
        assert_eq!(result, 0);
    }
}
