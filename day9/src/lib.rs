use rayon::prelude::*;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coord {
    x: u64,
    y: u64,
}

#[must_use]
pub fn parse_coords(input: &str) -> Vec<Coord> {
    let mut coords = vec![];

    for line in input.split_whitespace() {
        if let Some((x, y)) = line.split_once(',')
            && let Ok(x) = x.parse::<u64>()
            && let Ok(y) = y.parse::<u64>()
        {
            coords.push(Coord { x, y });
        }
    }

    coords
}

#[must_use]
pub fn solve_part_1(coords: &[Coord]) -> u64 {
    let mut result = 0;
    for (index, coord_a) in coords.iter().enumerate().take(coords.len() - 1) {
        for coord_b in coords.iter().skip(index + 1) {
            let area = (coord_a.x.abs_diff(coord_b.x) + 1) * (coord_a.y.abs_diff(coord_b.y) + 1);
            if result < area {
                result = area;
            }
        }
    }
    result
}

fn is_inside(vertical_segments: &[(Coord, Coord)], point: &Coord) -> bool {
    let mut intersects = 0;
    for vertical_segment in vertical_segments {
        if vertical_segment.0.x > point.x
            && (vertical_segment.0.y > point.y) != (vertical_segment.1.y > point.y)
        {
            intersects += 1;
        }
    }

    intersects % 2 == 1
}

fn create_segments(coords: &[Coord]) -> Vec<(Coord, Coord)> {
    let mut segments = vec![];
    let mut prev_coord = coords.last();
    for coord in coords {
        if let Some(prev_coord) = prev_coord {
            let segment = (*prev_coord, *coord);
            segments.push(segment);
        }
        prev_coord = Some(coord);
    }

    segments
}

fn create_line(segment: &(Coord, Coord)) -> Vec<Coord> {
    let mut line = vec![];
    if segment.0.x == segment.1.x {
        let x = segment.0.x;
        let y_range = if segment.0.y <= segment.1.y {
            (segment.0.y + 1)..(segment.1.y + 1)
        } else {
            (segment.1.y)..segment.0.y
        };
        for y in y_range {
            line.push(Coord { x, y });
        }
    } else if segment.0.y == segment.1.y {
        let y = segment.0.y;
        let x_range = if segment.0.x <= segment.1.x {
            (segment.0.x + 1)..(segment.1.x + 1)
        } else {
            segment.1.x..segment.0.x
        };
        for x in x_range {
            line.push(Coord { x, y });
        }
    }

    line
}

fn create_border_tiles(segments: &[(Coord, Coord)]) -> Vec<Coord> {
    let mut border_tiles = vec![];
    for segment in segments {
        let mut line = create_line(segment);
        border_tiles.append(&mut line);
    }

    border_tiles
}

fn is_valid_tile(
    border_tiles: &HashSet<Coord>,
    vertical_segments: &[(Coord, Coord)],
    tile: &Coord,
) -> bool {
    border_tiles.contains(tile) || is_inside(vertical_segments, tile)
}

fn get_min_max(rectangle: &(Coord, Coord)) -> (Coord, Coord) {
    let (min_x, max_x) = if rectangle.0.x <= rectangle.1.x {
        (rectangle.0.x, rectangle.1.x)
    } else {
        (rectangle.1.x, rectangle.0.x)
    };
    let (min_y, max_y) = if rectangle.0.y <= rectangle.1.y {
        (rectangle.0.y, rectangle.1.y)
    } else {
        (rectangle.1.y, rectangle.0.y)
    };

    (Coord { x: min_x, y: min_y }, Coord { x: max_x, y: max_y })
}

fn is_valide_rectangle(
    border_tiles: &HashSet<Coord>,
    vertical_segments: &[(Coord, Coord)],
    rectangle: &(Coord, Coord),
) -> bool {
    let (min, max) = get_min_max(rectangle);
    if !is_valid_tile(
        border_tiles,
        vertical_segments,
        &Coord {
            x: rectangle.0.x,
            y: rectangle.1.y,
        },
    ) || !is_valid_tile(
        border_tiles,
        vertical_segments,
        &Coord {
            x: rectangle.1.x,
            y: rectangle.0.y,
        },
    ) {
        return false;
    }

    (min.y..=max.y).into_par_iter().all(|y| {
        (min.x..=max.x).all(|x| {
            let tile = Coord { x, y };
            is_valid_tile(border_tiles, vertical_segments, &tile)
        })
    })
}

fn create_rectangles(red_coords: &[Coord]) -> Vec<(Coord, Coord, u64)> {
    let mut rectangles = vec![];
    for (index, coord_a) in red_coords.iter().enumerate().take(red_coords.len() - 1) {
        for coord_b in red_coords.iter().skip(index + 1) {
            let rectangle = (*coord_a, *coord_b);
            let area = (rectangle.0.x.abs_diff(rectangle.1.x) + 1)
                * (rectangle.0.y.abs_diff(rectangle.1.y) + 1);
            let rectangle = (*coord_a, *coord_b, area);
            rectangles.push(rectangle);
        }
    }

    rectangles
}

fn is_contains_invalid_rectangle(
    invalid_rectangles: &[(Coord, Coord)],
    rectangle: &(Coord, Coord),
) -> bool {
    let (min, max) = get_min_max(rectangle);
    for invalid_rectangle in invalid_rectangles {
        if min.x <= invalid_rectangle.0.x
            && invalid_rectangle.0.x <= max.x
            && min.x <= invalid_rectangle.1.x
            && invalid_rectangle.1.x <= max.x
            && min.y <= invalid_rectangle.0.y
            && invalid_rectangle.0.y <= max.y
            && min.y <= invalid_rectangle.1.y
            && invalid_rectangle.1.y <= max.y
        {
            return true;
        }
    }
    false
}

#[must_use]
pub fn solve_part_2(red_coords: &[Coord]) -> u64 {
    let segments = create_segments(red_coords);
    let border_tiles = create_border_tiles(&segments);
    let border_tiles: HashSet<Coord> = border_tiles.iter().copied().collect();
    let mut vertical_segments = vec![];
    for segment in segments {
        if segment.0.x == segment.1.x {
            vertical_segments.push(segment);
        }
    }
    let mut result = 0;
    let mut rectangles = create_rectangles(red_coords);
    rectangles.sort_by_key(|rectangle| rectangle.2);
    let mut invalid_rectangles = vec![];
    for (point_a, point_b, area) in rectangles {
        let rectangle = (point_a, point_b);
        if !is_contains_invalid_rectangle(&invalid_rectangles, &rectangle) {
            if !is_valide_rectangle(&border_tiles, &vertical_segments, &rectangle) {
                invalid_rectangles.push(rectangle);
            } else if result < area {
                result = area;
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
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let input = parse_coords(input);
        let result = solve_part_1(&input);
        assert_eq!(result, 50);
    }

    #[test]
    fn example_solve_part_2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let input = parse_coords(input);
        let result = solve_part_2(&input);
        assert_eq!(result, 24);
    }
}
