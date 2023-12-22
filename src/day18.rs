use std::{fs, io::Write};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day18;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coords {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HoleType {
    Wall,
    Interior,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Hole {
    coords: Coords,
    hole_type: HoleType,
}

fn calculate_part_1_attempt_1(contents: String) -> usize {
    let input_commands = parse_input(contents);
    let exterior_holes = build_exterior_holes(input_commands);
    // let hole_rows = build_hole_rows(&exterior_holes);
    let mut current_grid = build_hole_grid(&exterior_holes);
    let start = Hole {
        // Random known interior point - cheating hehe
        coords: Coords { x: 207, y: 80 },
        hole_type: HoleType::Interior,
    };
    flood_fill(&mut current_grid, start);
    let interior_holes = find_interior_holes(&mut current_grid);
    output_hole_grid_to_file(&mut current_grid);

    interior_holes.len() + exterior_holes.len()
}

fn build_hole_grid(exterior_holes: &Vec<Hole>) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 500]; 500];
    for hole in exterior_holes {
        grid[hole.coords.y][hole.coords.x] = 'E';
    }
    grid
}

fn output_hole_grid_to_file(grid: &mut Vec<Vec<char>>) {
    // dump the grid in a file where each row is a new line
    let mut file = fs::File::create("outputs/18-grid.txt").expect("Unable to create file");
    for row in grid.clone() {
        let row_string: String = row.into_iter().collect();
        // if all ements equal . then skip
        if row_string == ".".repeat(row_string.len()) {
            continue;
        }
        file.write_all(row_string.as_bytes())
            .expect("Unable to write data");
        file.write_all("\n".as_bytes())
            .expect("Unable to write data");
    }
}

fn flood_fill(grid: &mut Vec<Vec<char>>, start: Hole) {
    let mut stack = vec![start];
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let visited_characters = vec!['E', 'I'];

    while let Some(hole) = stack.pop() {
        if hole.coords.x < grid.len()
            && hole.coords.y < grid[0].len()
            && !visited_characters.contains(&grid[hole.coords.x][hole.coords.y])
        {
            grid[hole.coords.x][hole.coords.y] = 'I'; // Mark as visited with 'F'
            for &(dx, dy) in &directions {
                let new_x = (hole.coords.x as i32 + dx) as usize;
                let new_y = (hole.coords.y as i32 + dy) as usize;
                let new_hole = Hole {
                    coords: Coords { x: new_x, y: new_y },
                    hole_type: hole.hole_type,
                };
                stack.push(new_hole);
            }
        }
    }
}

fn find_interior_holes(grid: &mut Vec<Vec<char>>) -> Vec<Hole> {
    let mut interior_holes: Vec<Hole> = Vec::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, character) in row.iter().enumerate() {
            if character == &'I' {
                let interior_hole = Hole {
                    coords: Coords { x, y },
                    hole_type: HoleType::Interior,
                };
                interior_holes.push(interior_hole);
            }
        }
    }
    interior_holes
}

fn build_exterior_holes(commands: Vec<(char, usize, String)>) -> Vec<Hole> {
    // Start far enough from 0,0 so do not hit it as want to keep as usize
    let starting_coords = Coords { x: 200, y: 200 };
    let mut current_coords = starting_coords;
    let mut holes: Vec<Hole> = Vec::new();
    for command in commands {
        for _ in 0..command.1 {
            match command.0 {
                'U' => current_coords.y += 1,
                'D' => current_coords.y -= 1,
                'L' => current_coords.x -= 1,
                'R' => current_coords.x += 1,
                _ => panic!("Invalid direction"),
            }
            let hole = Hole {
                coords: current_coords,
                hole_type: HoleType::Wall,
            };
            holes.push(hole);
        }
    }
    holes
}

fn parse_input(contents: String) -> Vec<(char, usize, String)> {
    let input_commands: Vec<(char, usize, String)> = contents
        .lines()
        .map(|line| {
            let mut split_line = line.split_whitespace();
            let direction = split_line.next().unwrap().chars().next().unwrap();
            let distance = split_line.next().unwrap().parse::<usize>().unwrap();
            let colour = split_line.next().unwrap().to_string();
            (direction, distance, colour)
        })
        .collect();
    input_commands
}

impl Solution for Day18 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let count = calculate_part_1_attempt_1(_parsed_input.to_string());
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
    fn check_day18_part1_case1() {
        assert_eq!(
            Day18::solve_part_one(
                "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"
            ),
            "249976".to_string()
        )
    }
}
