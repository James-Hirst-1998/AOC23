use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day13;

fn calculate_part_1(contents: String) -> usize {
    let valley_of_mirrors = convert_input_to_mirrors(contents);
    let smudge_value = 0;
    let (row_reflection_points, column_reflection_points) =
        find_reflection_points_of_all_mirrors(&valley_of_mirrors, smudge_value);
    let sum = sum_up_reflection_points(row_reflection_points, column_reflection_points);
    sum
}

fn calculate_part_2(contents: String) -> usize {
    let valley_of_mirrors = convert_input_to_mirrors(contents);
    let smudge_value = 1;
    let (row_reflection_points, column_reflection_points) =
        find_reflection_points_of_all_mirrors(&valley_of_mirrors, smudge_value);
    let sum = sum_up_reflection_points(row_reflection_points, column_reflection_points);
    sum
}

fn sum_up_reflection_points(
    row_reflection_points: Vec<usize>,
    column_reflection_points: Vec<usize>,
) -> usize {
    let mut sum = 0;
    for row_point in row_reflection_points {
        sum += row_point;
    }
    for column_point in column_reflection_points {
        sum += column_point * 100;
    }
    sum
}

fn find_reflection_points_of_all_mirrors(
    valley_of_mirrors: &Vec<Vec<String>>, smudge_value: usize
) -> (Vec<usize>, Vec<usize>) {
    let mut row_reflection_points = Vec::new();
    let mut column_reflection_points = Vec::new();
    for mirror in valley_of_mirrors {
        let row_reflection_point = find_reflection_point_in_mirror_rows(mirror, &smudge_value);
        if row_reflection_point != 1000 {
            row_reflection_points.push(row_reflection_point);
        }
        let rotated_mirror = reflect_mirror_diagonally(mirror);
        let column_reflection_point = find_reflection_point_in_mirror_rows(&rotated_mirror, &smudge_value);
        if column_reflection_point != 1000 {
            column_reflection_points.push(column_reflection_point);
        }
    }
    (row_reflection_points, column_reflection_points)
}

fn reflect_mirror_diagonally(input: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for line in input.iter() {
        for (index, character) in line.chars().enumerate() {
            if output.get(index).is_none() {
                output.push(String::new());
            }
            output[index].push(character);
        }
    }
    output
}

fn find_reflection_point_in_mirror_rows(mirror: &Vec<String>, smudge_value: &usize) -> usize {
    let mut possible_reflection_points: HashMap<usize, usize> = HashMap::new();
    let number_of_rows = mirror.len();
    for row in mirror.iter() {
        let reflection_points = find_reflection_points_in_row(row);
        for point in reflection_points {
            if possible_reflection_points.contains_key(&point) {
                possible_reflection_points
                    .insert(point, possible_reflection_points.get(&point).unwrap() + 1);
            } else {
                possible_reflection_points.insert(point, 1);
            }
        }
    }

    for (point, number) in possible_reflection_points {
        if number == number_of_rows - smudge_value {
            return point;
        }
    }
    return 1000;
}

fn find_reflection_points_in_row(row: &String) -> Vec<usize> {
    let mut possible_reflection_points = Vec::new();
    let length_of_row = row.len();
    for index in 0..length_of_row{
        if index == 0 {
            continue;
        }
        // at every index I want to split the line and reduce the length of both parts so they are the
        // same as the smallest part. Then i want to check if the first part is equal to the second part
        // reversed. If it is then I want to add the index to the possible reflection points.
        let first_part: String;
        let second_part: String;
        if index < (length_of_row / 2) +1 {
            first_part = row[0..index].to_string();
            second_part = row[index..index+first_part.len()].to_string();
        } else {
            first_part = row[index..length_of_row].to_string();
            second_part = row[index - first_part.len()..index].to_string();
        }
        if first_part == second_part.chars().rev().collect::<String>() {
            possible_reflection_points.push(index);
        }
    }
    possible_reflection_points
}

fn convert_input_to_mirrors(contents: String) -> Vec<Vec<String>> {
    let mut valley_of_mirrors: Vec<Vec<String>> = Vec::new();
    let mut current_mirror = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            valley_of_mirrors.push(current_mirror);
            current_mirror = Vec::new();
            continue;
        }
        current_mirror.push(line.to_string());
    }
    valley_of_mirrors.push(current_mirror);
    valley_of_mirrors
}

impl Solution for Day13 {
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
    // test currently broken
    fn check_day13_part1_case1() {
        assert_eq!(Day13::solve_part_one("#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"), "405".to_string())
    }
}
