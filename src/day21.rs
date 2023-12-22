use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day21;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Coords {
    x: usize,
    y: usize,
}

fn calculate_part_1(contents: String, number_of_steps: usize) -> usize {
    let moveable_positions = parse_input_to_moveable_positions(&contents);
    let start_coords = find_start_coords(&contents);
    let visited_coords = move_n_number_of_steps(&moveable_positions, start_coords, number_of_steps);
    visited_coords.len()
}

fn find_start_coords(contents: &String) -> Coords {
    contents
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Coords { x, y }, c))
        })
        .find(|&(_, c)| c == 'S')
        .map(|(coords, _)| coords)
        .unwrap_or_else(|| panic!("No start coords found"))
}

fn move_n_number_of_steps(
    moveable_positions: &HashMap<Coords, Vec<Coords>>,
    start_coords: Coords,
    n: usize,
) -> Vec<Coords> {
    let mut current_coords = Vec::new();
    current_coords.push(start_coords);
    for _number in 0..n {
        let mut new_coords = Vec::new();
        for coords in current_coords.clone() {
            let adjacent_coords = moveable_positions.get(&coords).unwrap();
            for adjacent_coord in adjacent_coords {
                if !new_coords.contains(adjacent_coord) {
                    new_coords.push(adjacent_coord.clone());
                }
            }
        }
        current_coords = new_coords;
    }
    current_coords
}

fn parse_input_to_moveable_positions(contents: &String) -> HashMap<Coords, Vec<Coords>> {
    let rock_coords = find_rock_coords(&contents);
    let mut moveable_positions = HashMap::new();

    let width = contents.lines().next().unwrap().len();
    let height = contents.lines().count();

    let coords = (0..width).flat_map(|x| (0..height).map(move |y| Coords { x, y }));

    for coord in coords {
        if !rock_coords.contains(&coord) {
            let adjacent_places = find_all_adjacent_places(&coord, &width, &height);
            let filtered_adjacent_places = filtered_adjacent_places(adjacent_places, &rock_coords);
            moveable_positions.insert(coord, filtered_adjacent_places);
        }
    }
    moveable_positions
}

fn filtered_adjacent_places(
    adjacent_places: Vec<Coords>,
    rock_coords: &Vec<Coords>,
) -> Vec<Coords> {
    let mut filtered_adjacent_places = Vec::new();
    for coords in adjacent_places {
        if !rock_coords.contains(&coords) {
            filtered_adjacent_places.push(coords);
        }
    }
    filtered_adjacent_places
}

fn find_all_adjacent_places(coords: &Coords, width: &usize, height: &usize) -> Vec<Coords> {
    let mut adjacent_places = Vec::new();
    let x = coords.x;
    let y = coords.y;
    if x > 0 {
        adjacent_places.push(Coords { x: x - 1, y });
    }
    if x < width - 1 {
        adjacent_places.push(Coords { x: x + 1, y });
    }
    if y > 0 {
        adjacent_places.push(Coords { x, y: y - 1 });
    }
    if y < height - 1 {
        adjacent_places.push(Coords { x, y: y + 1 });
    }
    adjacent_places
}

fn find_rock_coords(contents: &String) -> Vec<Coords> {
    let mut rock_coords = Vec::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                rock_coords.push(Coords { x, y });
            }
        }
    }
    rock_coords
}

impl Solution for Day21 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let count = calculate_part_1(_parsed_input.to_string(), 64);
        count.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part two
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day21_part1_case1() {
        assert_eq!(
            Day21::solve_part_one(
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
            ),
            "42".to_string()
        )
    }
}
