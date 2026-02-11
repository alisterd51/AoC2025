pub struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
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

// https://en.wikipedia.org/wiki/Stars_and_bars_(combinatorics)
pub struct WeakCompositions {
    size: usize,
    n: usize,
    k: usize,
    comb: Option<Vec<usize>>,
}

#[allow(clippy::cast_possible_truncation)]
impl WeakCompositions {
    #[must_use]
    pub fn new(sum: u64, size: usize) -> Self {
        if size == 0 {
            return Self {
                size,
                n: 0,
                k: 0,
                comb: None,
            };
        }
        let n = sum as usize + size - 1;
        let k = size - 1;
        let comb = if k == 0 {
            Some(vec![])
        } else {
            Some((0..k).collect())
        };

        Self { size, n, k, comb }
    }
}

#[allow(clippy::cast_sign_loss)]
impl Iterator for WeakCompositions {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        let comb = self.comb.as_mut()?;
        let mut prev: isize = -1;
        let mut counts = Vec::with_capacity(self.size);

        for &b in comb.iter() {
            counts.push((b.cast_signed() - prev - 1) as u64);
            prev = b.cast_signed();
        }

        counts.push((self.n.cast_signed() - prev - 1) as u64);
        let mut moved = false;
        for i in (0..self.k).rev() {
            if comb[i] != i + self.n - self.k {
                comb[i] += 1;
                for j in i + 1..self.k {
                    comb[j] = comb[j - 1] + 1;
                }
                moved = true;
                break;
            }
        }
        if !moved {
            self.comb = None;
        }

        Some(counts)
    }
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

fn solve_machine_1(machine: &Machine) -> u64 {
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

#[allow(clippy::cast_precision_loss)]
fn solve(buttons: &[Vec<usize>], c: &[u64], r: u64) -> Option<Vec<u64>> {
    let n_counters = c.len();
    let m_buttons = buttons.len();
    let rows = n_counters + 1;
    let cols = m_buttons + 1;
    let mut matrix = vec![vec![0.0f64; cols]; rows];
    for (btn_idx, affected_counters) in buttons.iter().enumerate() {
        for &c_idx in affected_counters {
            matrix[c_idx][btn_idx] = 1.0;
        }
    }
    for i in 0..n_counters {
        matrix[i][m_buttons] = c[i] as f64;
    }
    for j in 0..m_buttons {
        matrix[n_counters][j] = 1.0;
    }
    matrix[n_counters][m_buttons] = r as f64;
    let mut pivot_row = 0;
    let mut pivot_cols = vec![-1isize; rows];
    let mut is_free_var = vec![true; m_buttons];
    for col in 0..m_buttons {
        if pivot_row >= rows { break; }
        let mut selected_row = pivot_row;
        while selected_row < rows && matrix[selected_row][col].abs() < 1e-9 {
            selected_row += 1;
        }
        if selected_row < rows {
            matrix.swap(pivot_row, selected_row);
            pivot_cols[pivot_row] = col.cast_signed();
            is_free_var[col] = false;
            let divisor = matrix[pivot_row][col];
            for j in col..cols {
                matrix[pivot_row][j] /= divisor;
            }
            for i in 0..rows {
                if i != pivot_row {
                    let factor = matrix[i][col];
                    if factor.abs() > 1e-9 {
                        for j in col..cols {
                            matrix[i][j] -= factor * matrix[pivot_row][j];
                        }
                    }
                }
            }
            pivot_row += 1;
        }
    }
    let free_indices: Vec<usize> = is_free_var.iter()
        .enumerate()
        .filter_map(|(i, &is_free)| if is_free { Some(i) } else { None })
        .collect();
    let mut result_x = vec![0.0f64; m_buttons];
    if solve_free_vars(0, &free_indices, &matrix, &pivot_cols, &mut result_x, r) {
        Some(result_x.iter().map(|&x| x.round() as u64).collect())
    } else {
        None
    }
}

fn solve_free_vars(
    idx: usize,
    free_indices: &[usize],
    matrix: &[Vec<f64>],
    pivot_cols: &[isize],
    x: &mut [f64],
    limit_r: u64
) -> bool {
    if idx == free_indices.len() {
        for r in (0..matrix.len()).rev() {
            let p_col = pivot_cols[r];
            if p_col != -1 {
                let p_col = p_col as usize;
                let mut sum = 0.0;
                let constant = matrix[r][matrix[0].len() - 1];
                for c in (p_col + 1)..x.len() {
                    sum += matrix[r][c] * x[c];
                }
                
                let val = constant - sum;
                if val < -1e-5 || (val.round() - val).abs() > 1e-5 {
                    return false;
                }
                x[p_col] = val.round();
            } else {
                let constant = matrix[r][matrix[0].len() - 1];
                 if constant.abs() > 1e-5 { return false; }
            }
        }
        return true;
    }
    let current_free = free_indices[idx];
    for val in 0..=limit_r {
        x[current_free] = val as f64;
        if solve_free_vars(idx + 1, free_indices, matrix, pivot_cols, x, limit_r) {
            return true;
        }
    }

    false
}

#[must_use]
pub fn solve_part_1(machines: &[Machine]) -> u64 {
    let mut result = 0;
    for machine in machines {
        result += solve_machine_1(machine);
    }

    result
}

fn solve_machine_2(machine: &Machine) -> u64 {
    let mut result = 0;
    let sum_joltages: u64 = machine.joltages.iter().sum();
    for i in 0..=sum_joltages {
        if let Some(r) = solve(&machine.buttons, &machine.joltages, i) {
            result = r.iter().sum();
            break;
        }
    }

    result
}

#[must_use]
pub fn solve_part_2(machines: &[Machine]) -> u64 {
    let mut result = 0;
    for machine in machines {
        result += solve_machine_2(machine);
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

    #[test]
    fn example_solve_part_2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let input = parse_machines(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 10 + 12 + 11);
    }

    #[test]
    fn custom_solve_part_2() {
        let input = "[...] (0) (1) {0,0,0}";
        let input = parse_machines(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 0);
        let input = "[...] (0) (1) (0) (1) {0,0,0}";
        let input = parse_machines(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 0);
        let input = "[...] (0) (1) (2) {0,0,1}";
        let input = parse_machines(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 1);
        let input = "[...] (0) (1) (2) {1,1,1}";
        let input = parse_machines(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 3);
        let input = "[...] (0,1,2) {1,1,1}";
        let input = parse_machines(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 1);
        let input = "[...] (0,1,2) {2,2,2}";
        let input = parse_machines(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 2);
        let input = "[...] (0) (1) (2) (0,1,2) {2,2,2}";
        let input = parse_machines(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 2);
    }
}
