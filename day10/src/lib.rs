pub struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    #[allow(dead_code)]
    joltages: Vec<u64>,
}

#[must_use]
pub fn parse_machines(input: &str) -> Vec<Machine> {
    let mut machines = vec![];
    for line in input.lines() {
        let mut lights = vec![];
        let mut buttons = vec![];
        let mut joltages = vec![];
        for word in line.split_whitespace() {
            if word.starts_with('[') && word.ends_with(']') {
                for light in word[1..word.len() - 1].chars() {
                    match light {
                        '.' => lights.push(false),
                        '#' => lights.push(true),
                        _ => {}
                    }
                }
            }
            if word.starts_with('(') && word.ends_with(')') {
                let button = &word[1..word.len() - 1];
                let mut affected_lights = vec![];
                for affected_light in button.split(',') {
                    if let Ok(affected_light) = affected_light.parse::<usize>() {
                        affected_lights.push(affected_light);
                    }
                }
                buttons.push(affected_lights);
            }
            if word.starts_with('{') && word.ends_with('}') {
                for joltage in word[1..word.len() - 1].split(',') {
                    if let Ok(joltage) = joltage.parse::<u64>() {
                        joltages.push(joltage);
                    }
                }
            }
        }

        let machine = Machine {
            lights,
            buttons,
            joltages,
        };
        machines.push(machine);
    }

    machines
}

#[allow(clippy::cast_possible_truncation)]
fn create_combos(n: usize) -> Vec<Vec<bool>> {
    let mut combos = vec![];
    let mut combo = vec![false; n];

    for _ in 0..2usize.pow(n as u32) {
        combos.push(combo.clone());
        for b in combo.iter_mut().take(n) {
            if !*b {
                *b = true;
                break;
            }
            *b = false;
        }
    }

    combos
}

fn solve_machine(machine: &Machine) -> u64 {
    let mut result = machine.buttons.len() as u64;
    let combos = create_combos(machine.buttons.len());

    for combo in combos {
        let mut lights = vec![false; machine.lights.len()];
        for affected_button in combo.iter().enumerate() {
            if *affected_button.1 {
                for affected_light in &machine.buttons[affected_button.0] {
                    lights[*affected_light] = !lights[*affected_light];
                }
            }
        }
        if lights == machine.lights {
            let mut sum = 0;
            for affected_button in combo {
                if affected_button {
                    sum += 1;
                }
            }
            if sum <= result {
                result = sum;
            }
        }
    }

    result
}

#[must_use]
pub fn solve_part_1(machines: &[Machine]) -> u64 {
    let mut result = 0;
    for machine in machines {
        result += solve_machine(machine);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let input = parse_machines(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 2 + 3 + 2);
    }
}
