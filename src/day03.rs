use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day03;

fn sum_of_parts(contents: String) -> (usize, usize) {
    let (all_engine_parts, all_gears) = find_all_engine_parts(contents);

    let engine_parts_sum = all_engine_parts.iter().sum();

    let mut gear_ratios = Vec::new();
    for gear in all_gears.values() {
        if gear.len() == 2 {
            let gear_ratio = gear[0] * gear[1];
            gear_ratios.push(gear_ratio)
        }
    }

    let gear_ratios_sum = gear_ratios.iter().sum();

    (engine_parts_sum, gear_ratios_sum)
}

fn find_all_engine_parts(contents: String) -> (Vec<usize>, HashMap<(usize, usize), Vec<usize>>) {
    let mut engine_parts: Vec<usize> = Vec::new();
    let lines = contents.split("\n");
    let line_vectors: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let static_line_vectors = line_vectors.clone();
    let mut gear_parts: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    let max_index_of_lines: i32 = (line_vectors.len() - 1).try_into().unwrap();
    let max_index_of_length: i32 = (line_vectors[0].len() - 1).try_into().unwrap();

    let mut current_line_number = 0;
    for line in line_vectors {
        let mut current_char_number = 0;
        let mut current_number = String::new();
        for char in line {
            let mut check_current_number = false;
            if char.is_numeric() {
                current_number.push(char);
                if current_char_number == max_index_of_length {
                    check_current_number = true
                }
            } else {
                if !current_number.is_empty() {
                    check_current_number = true
                }
            }

            if check_current_number {
                let current_num_length: i32 = current_number.len().try_into().unwrap();
                let mut start_width_position: i32 = (current_char_number - current_num_length - 1)
                    .try_into()
                    .unwrap();
                if start_width_position < 0 {
                    start_width_position = 0;
                }

                let end_position_width: i32 = current_char_number.try_into().unwrap();

                let mut start_position_height: i32 = (current_line_number - 1).try_into().unwrap();
                let mut end_position_height: i32 = (current_line_number + 1).try_into().unwrap();
                if start_position_height < 0 {
                    start_position_height = 0
                }
                if end_position_height > max_index_of_lines {
                    end_position_height = max_index_of_lines
                }

                let mut region_to_check: HashMap<char, HashMap<&str, usize>> = HashMap::new();
                for lin_num in start_position_height..end_position_height + 1 {
                    for char_num in start_width_position..end_position_width + 1 {
                        let lin_index: usize = lin_num.try_into().unwrap();
                        let char_index: usize = char_num.try_into().unwrap();
                        let mut region_data = HashMap::new();
                        region_data.insert("line_index", lin_index);
                        region_data.insert("char_index", char_index);
                        region_to_check
                            .insert(static_line_vectors[lin_index][char_index], region_data);
                    }
                }
                if contains_non_alphanumeric(&region_to_check.clone()) {
                    engine_parts.push(current_number.parse().unwrap())
                }

                if contains_a_gear(&region_to_check.clone()) {
                    let gear_info = region_to_check.get(&'*').unwrap().to_owned();
                    let gear_line_index = gear_info.get("line_index").unwrap().to_owned();
                    let gear_char_index = gear_info.get("char_index").unwrap().to_owned();
                    let gear_position = (gear_line_index, gear_char_index);

                    match gear_parts.get_mut(&gear_position) {
                        Some(v) => v.push(current_number.parse().unwrap()),
                        None => {
                            let v = vec![current_number.parse().unwrap()];
                            gear_parts.insert(gear_position, v);
                        }
                    }
                }

                current_number = String::new()
            }
            current_char_number += 1
        }
        current_line_number += 1
    }

    (engine_parts, gear_parts)
}

fn contains_non_alphanumeric(region: &HashMap<char, HashMap<&str, usize>>) -> bool {
    for c in region.keys() {
        if !c.is_alphanumeric() {
            return true;
        }
    }
    false
}

fn contains_a_gear(region: &HashMap<char, HashMap<&str, usize>>) -> bool {
    for c in region.keys() {
        if c == &'*' {
            return true;
        }
    }
    false
}

impl Solution for Day03 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let contents = input_lines.replace(".", "a"); // update .'s so can check if alphanumeric
        contents.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let (engine_sum, _gear_sum) = sum_of_parts(_parsed_input.to_string());
        engine_sum.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let (_engine_sum, gear_sum) = sum_of_parts(_parsed_input.to_string());
        gear_sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(
            Day03::solve(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
                false
            ),
            ("4361".to_string(), "467835".to_string())
        )
    }
}
