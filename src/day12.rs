// use itertools::repeat_n;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day12;

fn calculate_part_1(contents: String) -> usize {
    let spring_maps = parse_input(contents);
    let mut count = 0;
    for spring_map in spring_maps {
        count += spring_map.count_allowed_arrangement();
    }
    count
}

// fn calculate_part_2(contents: String) -> usize {
//     let spring_map = parse_input(contents);
//     for i in 1..=5 {
//         let updated_spring_map = multiply_spring_map_entries(spring_map.clone(), i);
//         let allowed_arrangements = find_allowed_arrangements(updated_spring_map);
//         let count = count_allowed_arrangement(allowed_arrangements);
//         println!("Multiple value: {}, Count: {}", i, count);
//     }
//     // let updated_spring_map = multiply_spring_map_entries(spring_map, 2);
//     // let allowed_arrangements = find_allowed_arrangements(updated_spring_map);
//     // let count = count_allowed_arrangement(allowed_arrangements);
//     // count
//     0
// }

fn parse_input(contents: String) -> Vec<SpringMap> {
    let mut spring_maps = Vec::new();
    for (index, line) in contents.lines().enumerate() {
        let mut split = line.split_whitespace();

        let spring_records = split.next().unwrap().to_string();
        let spring_pattern = split.next().unwrap();

        let spring_map = SpringMap::new(
            spring_records,
            spring_pattern
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
            index,
        );
        spring_maps.push(spring_map);
    }

    spring_maps
}

pub struct SpringMap {
    spring_records: String,
    spring_patterns: Vec<usize>,
    additional_spaces_allowed: usize,
    // line_number: usize,
}

impl SpringMap {
    fn new(spring_records: String, spring_patterns: Vec<usize>, _index: usize) -> SpringMap {
        let line_length = spring_records.len();
        let additional_spaces_allowed =
            line_length + 1 - spring_patterns.iter().sum::<usize>() - spring_patterns.len();
        SpringMap {
            spring_records,
            spring_patterns,
            additional_spaces_allowed,
            // line_number: index,
        }
    }

    // fn multiply_spring_map_entries(&self, multiple_value: usize) -> SpringMap {
    //     let new_string_records = vec![self.spring_records.as_str(); multiple_value].join("?");
    //     let new_spring_patterns: Vec<usize> =
    //         repeat_n(self.spring_patterns.clone(), multiple_value)
    //             .flatten()
    //             .collect();
    //     SpringMap::new(new_string_records, new_spring_patterns, self.line_number)
    // }

    fn count_allowed_arrangement(&self) -> usize {
        let allowed_arrangements = self.find_allowed_arrangements();
        allowed_arrangements.len()
    }

    fn find_allowed_arrangements(&self) -> Vec<String> {
        let possible_combinations = self.find_all_possible_combinations_of_pattern();
        let mut allowed_arrangements = Vec::new();
        for combination in possible_combinations {
            if self.is_allowed_arrangement(combination.clone()) {
                allowed_arrangements.push(combination);
            }
        }
        allowed_arrangements
    }

    fn find_all_possible_combinations_of_pattern(&self) -> Vec<String> {
        let mut possible_combinations = Vec::new();
        let number_of_space_entries = self.spring_patterns.len() + 1;
        let space_combinations = self
            .find_all_space_combinations(number_of_space_entries, self.additional_spaces_allowed);
        for space_combination in space_combinations {
            let arrangement = self.build_arrangement_from_empty_space(space_combination);
            possible_combinations.push(arrangement);
        }
        possible_combinations
    }

    fn find_all_space_combinations(
        &self,
        number_of_space_entries: usize,
        additional_spaces_allowed: usize,
    ) -> Vec<Vec<usize>> {
        let mut possible_combinations = Vec::new();

        if additional_spaces_allowed == 0 {
            let mut no_spaces_allowed = Vec::new();
            for _ in 0..number_of_space_entries {
                no_spaces_allowed.push(0);
            }
            possible_combinations.push(no_spaces_allowed);
        } else if number_of_space_entries == 1 {
            possible_combinations.push(vec![additional_spaces_allowed]);
        } else {
            for i in 0..=additional_spaces_allowed {
                let possible_combinations_end = self.find_all_space_combinations(
                    number_of_space_entries - 1,
                    additional_spaces_allowed - i,
                );
                for combination in possible_combinations_end {
                    let combination = [vec![i], combination].concat();
                    possible_combinations.push(combination);
                }
            }
        }
        possible_combinations
    }

    fn build_arrangement_from_empty_space(&self, space_combination: Vec<usize>) -> String {
        let mut arrangement = String::new();
        for (index, pattern) in space_combination.iter().enumerate() {
            let space_characters = ".".repeat(*pattern);
            arrangement.push_str(&space_characters);
            if index < self.spring_patterns.len() {
                let spring_pattern_characters = "#".repeat(self.spring_patterns[index]);
                arrangement.push_str(&spring_pattern_characters);
                arrangement.push_str(".");
            }
        }
        arrangement
    }

    fn is_allowed_arrangement(&self, combination: String) -> bool {
        let mut is_allowed = true;
        let combination_length = combination.len();
        for (index, character) in self.spring_records.chars().enumerate() {
            if index >= combination_length {
                break;
            } else if character == '?' {
                continue;
            } else if character != combination.chars().nth(index).unwrap() {
                is_allowed = false;
                break;
            }
        }
        is_allowed
    }
}

impl Solution for Day12 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let count = calculate_part_1(_parsed_input.to_string());
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
    fn check_day12_part1_case1() {
        assert_eq!(
            Day12::solve_part_one(
                "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"
            ),
            "21".to_string()
        )
    }

    #[test]
    fn check_day12_part2_case1() {
        assert_eq!(Day12::solve_part_two(""), "0".to_string())
    }
}
