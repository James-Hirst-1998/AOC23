use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day16;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Light {
    coord: (usize, usize),
    direction: Direction,
}

impl Light {
    pub fn new(coord: (usize, usize), direction: Direction) -> Self {
        Light { coord, direction }
    }
}

fn calculate_part_1(contents: String) -> usize {
    let grid_of_mirrors = parse_input(contents);

    let starting_coord = (0, 0);
    let starting_direction = Direction::Right;
    let starting_light = vec![Light::new(starting_coord, starting_direction)];

    let path_of_light = find_light_coords_hit(&grid_of_mirrors, starting_light);
    let unique_coords = find_unique_coords(&path_of_light);
    unique_coords.len()
}

fn calculate_part_2(contents: String) -> usize {
    let grid_of_mirrors = parse_input(contents.clone());

    let width = contents.lines().next().unwrap().len();
    let height = contents.lines().count();
    let all_possible_starting_positions = find_all_possible_starting_positions(width, height);
    println!(
        "Number of starting positions: {:?}",
        all_possible_starting_positions.len()
    );

    let mut max_number_of_spaces_energized = 0;
    for (index, starting_position) in all_possible_starting_positions.iter().enumerate() {
        let path_of_light =
            find_light_coords_hit(&grid_of_mirrors, vec![starting_position.clone()]);
        let unique_coords = find_unique_coords(&path_of_light);
        let number_of_spaces_energized = unique_coords.len();
        println!("{}: {:?}", index, number_of_spaces_energized);
        if number_of_spaces_energized > max_number_of_spaces_energized {
            max_number_of_spaces_energized = unique_coords.len();
        }
    }
    max_number_of_spaces_energized
}

fn find_unique_coords(path_of_light: &Vec<Light>) -> Vec<(usize, usize)> {
    let mut unique_coords = Vec::new();
    for light in path_of_light {
        if !unique_coords.contains(&light.coord) {
            unique_coords.push(light.coord);
        }
    }
    unique_coords
}

fn find_all_possible_starting_positions(width: usize, height: usize) -> Vec<Light> {
    let mut starting_positions = Vec::new();
    for row in 0..height {
        starting_positions.push(Light::new((row, 0), Direction::Right));
        starting_positions.push(Light::new((row, width - 1), Direction::Left));
    }
    for col in 0..width {
        starting_positions.push(Light::new((0, col), Direction::Down));
        starting_positions.push(Light::new((height - 1, col), Direction::Up));
    }
    starting_positions
}

fn find_light_coords_hit(
    grid_of_mirrors: &HashMap<(usize, usize), char>,
    starting_light: Vec<Light>,
) -> Vec<Light> {
    let mut current_beams = starting_light.clone();

    let mut beams_already_hit = Vec::new();

    while !current_beams.is_empty() {
        let mut next_beams = Vec::new();
        for light_beam in current_beams {
            // println!("light_beam: {:?}", light_beam);
            if grid_of_mirrors.contains_key(&light_beam.coord) {
                let current_light_coord_entry = grid_of_mirrors.get(&light_beam.coord).unwrap();
                let outcome_directions = find_outcome_directions_from_next_coord(
                    &current_light_coord_entry,
                    &light_beam.direction,
                );
                for outcome_direction in outcome_directions {
                    let next_coord = find_next_coord(&light_beam.coord, &outcome_direction);
                    let next_light = Light::new(next_coord, outcome_direction);
                    if !beams_already_hit.contains(&next_light) {
                        // println!("New next_light: {:?}", next_light);
                        next_beams.push(next_light);
                    }
                }
                beams_already_hit.push(light_beam);
            }
        }
        // println!("");
        current_beams = next_beams;
    }
    beams_already_hit
}

fn find_outcome_directions_from_next_coord(
    coord_entry: &char,
    current_direction: &Direction,
) -> Vec<Direction> {
    let mut outcome_directions = Vec::new();
    match coord_entry {
        '/' => match current_direction {
            Direction::Left => outcome_directions.push(Direction::Down),
            Direction::Right => outcome_directions.push(Direction::Up),
            Direction::Up => outcome_directions.push(Direction::Right),
            Direction::Down => outcome_directions.push(Direction::Left),
        },
        '\\' => match current_direction {
            Direction::Left => outcome_directions.push(Direction::Up),
            Direction::Right => outcome_directions.push(Direction::Down),
            Direction::Up => outcome_directions.push(Direction::Left),
            Direction::Down => outcome_directions.push(Direction::Right),
        },
        '-' => match current_direction {
            Direction::Left => outcome_directions.push(Direction::Left),
            Direction::Right => outcome_directions.push(Direction::Right),
            Direction::Up => {
                outcome_directions.push(Direction::Left);
                outcome_directions.push(Direction::Right);
            }
            Direction::Down => {
                outcome_directions.push(Direction::Left);
                outcome_directions.push(Direction::Right);
            }
        },
        '|' => match current_direction {
            Direction::Left => {
                outcome_directions.push(Direction::Up);
                outcome_directions.push(Direction::Down);
            }
            Direction::Right => {
                outcome_directions.push(Direction::Up);
                outcome_directions.push(Direction::Down);
            }
            Direction::Up => outcome_directions.push(Direction::Up),
            Direction::Down => outcome_directions.push(Direction::Down),
        },
        '.' => match current_direction {
            Direction::Left => outcome_directions.push(Direction::Left),
            Direction::Right => outcome_directions.push(Direction::Right),
            Direction::Up => outcome_directions.push(Direction::Up),
            Direction::Down => outcome_directions.push(Direction::Down),
        },
        _ => panic!("Unexpected character"),
    }
    outcome_directions
}

fn find_next_coord(
    current_coord: &(usize, usize),
    current_direction: &Direction,
) -> (usize, usize) {
    let mut new_coord: (isize, isize);

    match current_direction {
        Direction::Left => new_coord = (current_coord.0 as isize, (current_coord.1 as isize - 1)),
        Direction::Right => new_coord = (current_coord.0 as isize, (current_coord.1 + 1) as isize),
        Direction::Up => {
            new_coord = (
                (current_coord.0 as isize - 1) as isize,
                current_coord.1 as isize,
            )
        }
        Direction::Down => new_coord = ((current_coord.0 + 1) as isize, current_coord.1 as isize),
    }

    if new_coord.0 < 0 || new_coord.1 < 0 {
        new_coord = (1000, 1000);
    }

    (new_coord.0 as usize, new_coord.1 as usize)
}

fn parse_input(contents: String) -> HashMap<(usize, usize), char> {
    contents
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, c)| ((row, col), c))
        })
        .collect()
}

impl Solution for Day16 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let count = calculate_part_1(_parsed_input.clone());
        count.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        // let count = calculate_part_2(_parsed_input.clone());
        // count.to_string()
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test currently broken
    fn check_day16_both_case1() {
        assert_eq!(
            Day16::solve(
".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....",
                false
            ),
            ("46".to_string(), "51".to_string())
        )
    }
}
