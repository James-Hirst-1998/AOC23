use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day11;

fn calculate_total_distance_between_galaxies(contents: String, expansion_constant: u64) -> u64 {
    let mut total_distance = 0;
    let all_distances = calculate_distance_between_galaxies(contents, expansion_constant);
    for distance in all_distances {
        total_distance += distance;
    }
    return total_distance / 2;
}

fn calculate_distance_between_galaxies(contents: String, expansion_constant: u64) -> Vec<u64> {
    let galaxy_positions = work_out_galaxy_positions(&contents);
    let expanding_rows = find_expanding_rows(&contents);
    let expanding_cols = find_expanding_columns(&contents);

    let mut all_distances = Vec::new();
    for (row1, col1) in galaxy_positions.iter() {
        for (row2, col2) in galaxy_positions.iter() {
            let mut distance = 0;
            if row1 == row2 && col1 == col2 {
                continue;
            }
            let row_distance = if row1 > row2 {
                *row1 - *row2
            } else {
                *row2 - *row1
            };
            let col_distance = if col1 > col2 {
                *col1 - *col2
            } else {
                *col2 - *col1
            };
            distance += row_distance + col_distance;

            let mut number_of_expansions_hit: u64 = 0;
            for expanding_row in expanding_rows.iter() {
                if *row1 > *row2 {
                    if *expanding_row > *row2 && *expanding_row < *row1 {
                        number_of_expansions_hit += 1;
                    }
                } else {
                    if *expanding_row > *row1 && *expanding_row < *row2 {
                        number_of_expansions_hit += 1;
                    }
                }
            }
            for expanding_col in expanding_cols.iter() {
                if *col1 > *col2 {
                    if *expanding_col > *col2 && *expanding_col < *col1 {
                        number_of_expansions_hit += 1;
                    }
                } else {
                    if *expanding_col > *col1 && *expanding_col < *col2 {
                        number_of_expansions_hit += 1;
                    }
                }
            }
            distance += number_of_expansions_hit * expansion_constant;
            all_distances.push(distance);
        }
    }
    all_distances
}

fn work_out_galaxy_positions(contents: &String) -> Vec<(u64, u64)> {
    let mut galaxy_positions = Vec::new();
    let mut row_number = 0;
    for line in contents.lines() {
        let mut col_number = 0;
        for character in line.chars() {
            if character == '#' {
                galaxy_positions.push((row_number, col_number));
            }
            col_number += 1;
        }
        row_number += 1;
    }
    galaxy_positions
}

fn find_expanding_rows(contents: &String) -> Vec<u64> {
    let mut expanding_rows = Vec::new();
    let mut row_number = 0;
    for line in contents.lines() {
        if line.chars().all(|ch| ch == '.') {
            expanding_rows.push(row_number);
        }
        row_number += 1;
    }
    expanding_rows
}

fn find_expanding_columns(contents: &String) -> Vec<u64> {
    let mut expanding_columns = Vec::new();
    let mut counting_columns: HashMap<u64, u64> = HashMap::new();
    let line_length = contents.lines().next().map_or(0, |line| line.len());
    for line in contents.lines() {
        let mut col_number = 0;
        for char in line.chars() {
            if char == '.' {
                let count = counting_columns.entry(col_number).or_insert(0);
                *count += 1;
            }
            col_number += 1;
        }
    }

    for (key, value) in counting_columns.iter() {
        if *value == line_length as u64 {
            expanding_columns.push(*key);
        }
    }

    expanding_columns
}

impl Solution for Day11 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let total_distance =
            calculate_total_distance_between_galaxies(_parsed_input.to_string(), 1);
        total_distance.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let total_distance =
            calculate_total_distance_between_galaxies(_parsed_input.to_string(), 999999);
        total_distance.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test currently broken
    fn check_day11_both_case1() {
        assert_eq!(
            Day11::solve(
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                false
            ),
            ("374".to_string(), "82000210".to_string())
        )
    }
}
