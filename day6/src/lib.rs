pub enum Problem {
    Add(Vec<u64>),
    Multiply(Vec<u64>),
}

enum ParsedOps {
    Add,
    Multiply,
}

#[must_use]
pub fn parse_homework(input: &str) -> Vec<Problem> {
    let mut problems = vec![];
    let mut parsed_lines = vec![];
    let mut parsed_ops = vec![];

    for line in input.lines() {
        let mut parsed_line = vec![];

        for input in line.split_whitespace() {
            if let Ok(number) = input.parse::<u64>() {
                parsed_line.push(number);
            } else if input == "+" {
                parsed_ops.push(ParsedOps::Add);
            } else if input == "*" {
                parsed_ops.push(ParsedOps::Multiply);
            }
        }
        if !parsed_line.is_empty() {
            parsed_lines.push(parsed_line);
        }
    }
    for (index, ops) in parsed_ops.iter().enumerate() {
        let mut problem = vec![];

        for parsed_line in &parsed_lines {
            problem.push(parsed_line[index]);
        }
        match ops {
            ParsedOps::Add => problems.push(Problem::Add(problem)),
            ParsedOps::Multiply => problems.push(Problem::Multiply(problem)),
        }
    }

    problems
}

fn solve_problem(problem: &Problem) -> u64 {
    match problem {
        Problem::Add(items) => {
            let mut result = 0;
            for item in items {
                result += item;
            }
            result
        }
        Problem::Multiply(items) => {
            let mut result = 1;
            for item in items {
                result *= item;
            }
            result
        }
    }
}

#[must_use]
pub fn solve_part_1(problems: &[Problem]) -> u64 {
    let mut result = 0;
    for problem in problems {
        result += solve_problem(problem);
    }

    result
}

// #[must_use]
// pub fn solve_part_2(problems: &[Problem]) -> u64 {
//     let mut result = 0;
//     for problem in problems {
//         result += solve_problem(problem);
//     }

//     result
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let input = parse_homework(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn example_solve_part_2() {}
}
