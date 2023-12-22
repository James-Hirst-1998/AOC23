use std::collections::HashMap;
use geo::{point, Contains, Coord, LineString};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day10;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn equal(&self, other: &Coords) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn calculate_part_1(contents: String) -> usize {
    let loop_of_coords = find_loop_of_pipe_coords(&contents);
    (loop_of_coords.len() - 1) / 2
}

fn calculate_part_2(contents: String) -> usize {
    let loop_of_coords = find_loop_of_pipe_coords(&contents);
    let line_string = LineString(
        loop_of_coords
            .iter()
            .map(|coords| Coord {
                x: coords.x as f64,
                y: coords.y as f64,
            })
            .collect(),
    );
    let polygon = geo::Polygon::new(line_string, vec![]);

    let mut count = 0;
    for (row, line) in contents.lines().enumerate() {
        for (col, _character) in line.chars().enumerate() {
            if polygon.contains(&point!(x: col as f64, y: row as f64)) {
                count += 1;
            }
        }
    }
    count
}

fn find_loop_of_pipe_coords(contents: &String) -> Vec<Coords> {
    let (map, start) = compute_map_of_places(&contents);
    println!("Built map");
    let mut loop_of_coords = Vec::new();
    loop_of_coords.push(start);
    let first_direction = find_first_coords_which_touch_start(start, &map);
    loop_of_coords.push(first_direction);

    let mut current_position = first_direction;
    while !current_position.equal(&start) {
        let (first, second) = map.get(&current_position).unwrap();
        if Some(first) == loop_of_coords.get(loop_of_coords.len().saturating_sub(2)) {
            current_position = *second;
        } else {
            current_position = *first;
        }
        loop_of_coords.push(current_position);
    }
    loop_of_coords
}

fn compute_map_of_places(contents: &String) -> (HashMap<Coords, (Coords, Coords)>, Coords) {
    let mut map = HashMap::new();
    let mut start = Coords { x: 0, y: 0 };
    for (row, line) in contents.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            let coords = Coords { x: col, y: row };
            let value = match character {
                '|' => (
                    Coords { x: col, y: row + 1 },
                    Coords {
                        x: col,
                        y: row.saturating_sub(1),
                    },
                ),

                '-' => (
                    Coords { x: col + 1, y: row },
                    Coords {
                        x: col.saturating_sub(1),
                        y: row,
                    },
                ),

                'L' => (
                    Coords { x: col + 1, y: row },
                    Coords {
                        x: col,
                        y: row.saturating_sub(1),
                    },
                ),

                'J' => (
                    Coords {
                        x: col.saturating_sub(1),
                        y: row,
                    },
                    Coords {
                        x: col,
                        y: row.saturating_sub(1),
                    },
                ),

                'F' => (Coords { x: col + 1, y: row }, Coords { x: col, y: row + 1 }),
                '7' => (
                    Coords {
                        x: col.saturating_sub(1),
                        y: row,
                    },
                    Coords { x: col, y: row + 1 },
                ),
                'S' => {
                    start = Coords { x: col, y: row };
                    continue;
                }
                '.' => continue,
                _ => panic!("Invalid character"),
            };
            map.insert(coords, value);
        }
    }
    (map, start)
}

fn find_first_coords_which_touch_start(
    start: Coords,
    map: &HashMap<Coords, (Coords, Coords)>,
) -> Coords {
    let mut touching_coords = Coords { x: 0, y: 0 };
    map.iter().for_each(|(coords, (first, second))| {
        if *first == start || *second == start {
            touching_coords = *coords;
        }
    });
    touching_coords
}

impl Solution for Day10 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let part_1 = calculate_part_1(_parsed_input.to_string());
        part_1.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let part_2 = calculate_part_2(_parsed_input.to_string());
        part_2.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day10_part1_case1() {
        assert_eq!(Day10::solve_part_one("7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ"), "8".to_string())
    }

    #[test]
    fn check_day10_part2_case1() {
        assert_eq!(Day10::solve_part_two(".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ..."), "8".to_string())
    }
}
