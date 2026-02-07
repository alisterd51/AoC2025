fn solve_n(bank: &[u64], size: usize) -> u64 {
    let mut result = 0;
    let mut base_index = 0;
    for j in 0..size {
        let bank = &bank[base_index..(bank.len() - (size - 1 - j))];
        let mut index_max = 0;
        let mut batterie_max = 0;
        for (index, batterie) in bank.iter().enumerate() {
            if batterie_max < *batterie {
                index_max = index;
                batterie_max = *batterie;
            }
        }
        result = result * 10 + batterie_max;
        base_index += index_max + 1;
    }
    result
}

#[must_use]
pub fn solve_part_1(input: Vec<&str>) -> u64 {
    let mut result = 0;

    for bank in input {
        let mut bank_u64 = vec![];
        for batterie in bank.chars().filter_map(|c| c.to_digit(10).map(u64::from)) {
            bank_u64.push(batterie);
        }
        result += solve_n(&bank_u64, 2);
    }
    result
}

#[must_use]
pub fn solve_part_2(input: Vec<&str>) -> u64 {
    let mut result = 0;

    for bank in input {
        let mut bank_u64 = vec![];
        for batterie in bank.chars().filter_map(|c| c.to_digit(10).map(u64::from)) {
            bank_u64.push(batterie);
        }
        result += solve_n(&bank_u64, 12);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_solve_part_n() {
        let input = vec![1, 2];
        let result = solve_n(&input, 1);
        assert_eq!(result, 2);
        let input = vec![1, 2];
        let result = solve_n(&input, 2);
        assert_eq!(result, 12);
        let input = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        let result = solve_n(&input, 2);
        assert_eq!(result, 92);
    }

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
        let input = vec!["987654321111111"];
        let result = solve_part_2(input);
        assert_eq!(result, 987654321111);
        let input = vec!["811111111111119"];
        let result = solve_part_2(input);
        assert_eq!(result, 811111111119);
        let input = vec!["234234234234278"];
        let result = solve_part_2(input);
        assert_eq!(result, 434234234278);
        let input = vec!["818181911112111"];
        let result = solve_part_2(input);
        assert_eq!(result, 888911112111);
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let result = solve_part_2(input);
        assert_eq!(result, 3121910778619);
    }
}
