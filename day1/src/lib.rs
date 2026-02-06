#[derive(Debug, PartialEq, Copy, Clone)]
enum Rotation {
    Left(u64),
    Right(u64),
}

fn convert_rotation(rotation: &str) -> Option<Rotation> {
    match rotation.as_bytes().first() {
        Some(b'L') => rotation[1..]
            .parse::<u64>()
            .map_or(None, |rotate| Some(Rotation::Left(rotate))),
        Some(b'R') => rotation[1..]
            .parse::<u64>()
            .map_or(None, |rotate| Some(Rotation::Right(rotate))),
        _ => None,
    }
}

#[must_use]
pub fn solve_part_1(input: Vec<&str>) -> u64 {
    let mut rotations = vec![];
    for rotation in input {
        if let Some(rotation) = convert_rotation(rotation) {
            rotations.push(rotation);
        }
    }
    let mut result = 0;
    let mut dial = 50;
    for rotation in rotations {
        match rotation {
            Rotation::Left(rotate) => {
                let rotate = rotate % 100;
                if dial < rotate {
                    dial += 100 - rotate;
                } else {
                    dial -= rotate;
                }
                if dial == 0 {
                    result += 1;
                }
            }
            Rotation::Right(rotate) => {
                let rotate = rotate % 100;
                dial = (dial + rotate) % 100;
                if dial == 0 {
                    result += 1;
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
        let input = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        let result = solve_part_1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_convert_rotation() {
        let input = "L42";
        let result = convert_rotation(input);
        assert_eq!(result, Some(Rotation::Left(42)));
        let input = "R43";
        let result = convert_rotation(input);
        assert_eq!(result, Some(Rotation::Right(43)));
    }
}
