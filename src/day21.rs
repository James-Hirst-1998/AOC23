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
    println!("Start coords: {:?}", start_coords);
    let visited_coords = move_n_number_of_steps(&moveable_positions, start_coords, number_of_steps);
    visited_coords.len()
}

// I wanted to do something with covering complete grids because you move out like a diamond
// each step but I cannot work out what is going wrong - U ave a list of my guesses at the bottom
fn calculate_part_2(contents: String, number_of_steps: usize) -> usize {
    let width_of_grid = contents.lines().next().unwrap().len();
    let height_of_grid = contents.lines().count();
    let number_of_rocks_per_grid = find_rock_coords(&contents).len();

    if width_of_grid != height_of_grid {
        panic!("Width and height of grid must be equal");
    }

    let number_of_steps_left = number_of_steps % width_of_grid;
    let number_of_full_grids = (number_of_steps - number_of_steps_left) / 4 * width_of_grid;
    let gardens_reached_in_full_grid_on_odd = (width_of_grid * width_of_grid) - number_of_rocks_per_grid -4;
    let number_of_spaces_in_full_grids =
        number_of_full_grids * gardens_reached_in_full_grid_on_odd;
    let number_of_spaces_in_partial_grid =
        find_number_of_places_hit_moving_from_edge_midpoints_n_places(
            &parse_input_to_moveable_positions(&contents),
            &width_of_grid,
            number_of_steps_left,
        );
    // let number_of_spaces_in_partial_grid = 7688;

    println!("Number of steps left: {}", number_of_steps_left);
    println!("Number of full grids: {}", number_of_full_grids);
    println!(
        "Number of spaces in full grids: {}",
        number_of_spaces_in_full_grids
    );
    println!(
        "Number of spaces in partial grid: {}",
        number_of_spaces_in_partial_grid
    );

    number_of_spaces_in_full_grids + number_of_spaces_in_partial_grid
}

fn find_number_of_places_hit_moving_from_edge_midpoints_n_places(
    moveable_positions: &HashMap<Coords, Vec<Coords>>,
    width: &usize,
    number_of_steps: usize,
) -> usize {
    let mid_points = (width - 1) / 2;
    let midpoint_coords_of_sides = vec![
        Coords {
            x: 0,
            y: mid_points,
        },
        Coords {
            x: mid_points,
            y: 0,
        },
        Coords {
            x: width - 1,
            y: mid_points,
        },
        Coords {
            x: mid_points,
            y: width - 1,
        },
    ];
    println!("Midpoint coords: {:?}", midpoint_coords_of_sides);

    let mut all_end_places = Vec::new();
    for starting_position in midpoint_coords_of_sides {
        let end_places =
            move_n_number_of_steps(moveable_positions, starting_position, number_of_steps);
        all_end_places.extend(end_places);
    }
    all_end_places.len()
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
        // println!("Number {} - new coords: {:?}", _number, new_coords.len());
        current_coords = new_coords;
    }
    current_coords
}

fn parse_input_to_moveable_positions(contents: &String) -> HashMap<Coords, Vec<Coords>> {
    let rock_coords = find_rock_coords(&contents);
    println!("Number of rocks: {}", rock_coords.len());
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
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let count = calculate_part_1(_parsed_input.to_string(), 64);
        count.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let count = calculate_part_2(_parsed_input.to_string(), 26501365);
        count.to_string()
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

// Guesses for part 2
// 26634654549288
// 26634654549256
// 26634654556944
// 26634654549513
// 26634652539656
// 13345100641113

// Alternates between:
// Number 156 - new coords: 7656
// Number 157 - new coords: 7688
