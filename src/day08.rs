use num_integer::lcm;
use regex::Regex;
use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day08;

fn calculate_part_1(contents: String) -> usize {
    let instructions = calculate_instructions(contents.clone());
    let map = calculate_map(contents.clone());
    let mut hit_zzz = false;
    let mut places_moved = 0;
    let mut current_position = "AAA";
    while !hit_zzz {
        loop {
            let i = places_moved % instructions.len();
            let instruction = instructions.chars().nth(i).unwrap();

            if instruction == 'R' {
                current_position = &map.get(current_position).unwrap().1;
            } else if instruction == 'L' {
                current_position = &map.get(current_position).unwrap().0;
            }
            places_moved += 1;

            if current_position == "ZZZ" {
                hit_zzz = true;
                break;
            }
        }
    }
    places_moved
}

fn calculate_part_2(contents: String) -> usize {
    let instructions = calculate_instructions(contents.clone());
    let map = calculate_map(contents.clone());
    let mut starting_positions = Vec::new();
    for key in map.keys() {
        if key.ends_with("A") {
            starting_positions.push(key);
        }
    }

    let mut current_positions = starting_positions.clone();
    let mut places_moved = 0;
    let mut looping_values = Vec::new();
    while current_positions.len() > 0 {
        let i = places_moved % instructions.len();
        let instruction = instructions.chars().nth(i).unwrap();

        let mut new_positions = Vec::new();

        for current_position in current_positions.clone() {
            let mut new_position = current_position;
            if instruction == 'R' {
                new_position = &map.get(current_position).unwrap().1;
            } else if instruction == 'L' {
                new_position = &map.get(current_position).unwrap().0;
            }

            // If a value hits the end of its loop then do not carry on finding its values
            if new_position.ends_with("Z") {
                let new_places_moved = places_moved + 1;
                looping_values.push(new_places_moved);
            } else {
                new_positions.push(new_position);
            }
        }
        places_moved += 1;
        current_positions = new_positions;
    }

    lowest_common_multiple(looping_values)
}

fn calculate_instructions(contents: String) -> String {
    contents.lines().next().unwrap().to_string()
}

fn lowest_common_multiple(values: Vec<usize>) -> usize {
    values.iter().fold(1, |a, &b| lcm(a, b))
}

fn calculate_map(contents: String) -> HashMap<String, (String, String)> {
    let mut map = HashMap::new();
    let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

    for line in contents.lines().skip(1) {
        if line.is_empty() {
            continue;
        }

        if let Some(cap) = re.captures(&line) {
            let key = cap[1].to_string();
            let left = cap[2].to_string();
            let right = cap[3].to_string();
            map.insert(key, (left, right));
        }
    }
    map
}

impl Solution for Day08 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
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
    fn check_day08_part1_case1() {
        assert_eq!(
            Day08::solve_part_one(
                "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
            ),
            "6".to_string()
        )
    }

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(
            Day08::solve_part_two(
                "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"
            ),
            "6".to_string()
        )
    }
}
