pub struct Coord {
    x: u64,
    y: u64,
    z: u64,
}

#[must_use]
pub fn parse_coords(input: &str) -> Vec<Coord> {
    let mut coords = vec![];

    for line in input.split_whitespace() {
        if let Some((x, yz)) = line.split_once(',')
            && let Ok(x) = x.parse::<u64>()
            && let Some((y, z)) = yz.split_once(',')
            && let Ok(y) = y.parse::<u64>()
            && let Ok(z) = z.parse::<u64>()
        {
            coords.push(Coord { x, y, z });
        }
    }

    coords
}

fn create_circuit(junctions: &[(usize, usize)], node: usize) -> Vec<usize> {
    let mut circuit = vec![node];
    let mut new_nodes = true;
    while new_nodes {
        new_nodes = false;
        for junction in junctions {
            if circuit.contains(&junction.0) && !circuit.contains(&junction.1) {
                circuit.push(junction.1);
                new_nodes = true;
            } else if !circuit.contains(&junction.0) && circuit.contains(&junction.1) {
                circuit.push(junction.0);
                new_nodes = true;
            }
        }
    }

    circuit
}

fn is_directly_connected(junctions: &[(usize, usize)], junction: (usize, usize)) -> bool {
    junctions.contains(&junction) || junctions.contains(&(junction.1, junction.0))
}

fn create_shortest_junction(
    coords: &[Coord],
    junctions: &[(usize, usize)],
) -> Option<(usize, usize, f64)> {
    let mut shortest_junction = None;
    for (index, coord) in coords.iter().enumerate().take(coords.len() - 1) {
        for (other_index, other_coord) in coords.iter().enumerate().skip(index + 1) {
            if !is_directly_connected(junctions, (index, other_index)) {
                let distance = ((coord.x as f64 - other_coord.x as f64).powi(2)
                    + (coord.y as f64 - other_coord.y as f64).powi(2)
                    + (coord.z as f64 - other_coord.z as f64).powi(2))
                .sqrt();
                match shortest_junction {
                    Some((_, _, shortest_distance)) => {
                        if distance < shortest_distance {
                            shortest_junction = Some((index, other_index, distance));
                        }
                    }
                    None => shortest_junction = Some((index, other_index, distance)),
                }
            }
        }
    }

    shortest_junction
}

#[must_use]
pub fn solve_part_1(coords: &[Coord], mut to_connected: u64) -> u64 {
    let mut junctions = vec![];
    while to_connected != 0
        && let Some(junction) = create_shortest_junction(coords, &junctions)
    {
        let junction = (junction.0, junction.1);
        junctions.push(junction);
        to_connected -= 1;
    }

    let mut circuits: Vec<Vec<usize>> = vec![];
    'outer: for index in 0..coords.len() {
        for circuit in &circuits {
            if circuit.contains(&index) {
                continue 'outer;
            }
        }
        let circuit = create_circuit(&junctions, index);
        circuits.push(circuit);
    }

    circuits.sort_by_key(|circuit| std::cmp::Reverse(circuit.len()));

    let top = &circuits[..3];

    (top[0].len() * top[1].len() * top[2].len()) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solve_part_1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let input = parse_coords(input);
        let result = solve_part_1(&input, 10);
        assert_eq!(result, 40);
    }
}
