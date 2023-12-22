use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day09;

fn calculate_numbers(contents: String, next: bool) -> i32 {
    let next_numbers = calculate_next_numbers_in_each_sequence(contents, next);
    next_numbers.iter().sum()
}

fn calculate_next_numbers_in_each_sequence(contents: String, next: bool) -> Vec<i32> {
    let mut next_numbers = Vec::new();
    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let mut all_sequences = calculate_differences(&numbers);
        all_sequences.push(numbers);

        let next_number = calculate_new_number_given_differences(&all_sequences, next);
        next_numbers.push(next_number);
    }
    next_numbers
}

fn calculate_differences(numbers: &Vec<i32>) -> Vec<Vec<i32>> {
    if are_all_elements_same(&numbers) {
        return vec![];
    }

    let mut differences = Vec::new();
    for i in 0..numbers.len() - 1 {
        differences.push(numbers[i + 1] - numbers[i]);
    }

    let diff_of_diffs = calculate_differences(&differences);
    let mut result = Vec::new();
    for diff in diff_of_diffs {
        result.push(diff)
    }
    result.push(differences);
    result
}

fn are_all_elements_same(numbers: &[i32]) -> bool {
    if let Some(first) = numbers.first() {
        for num in numbers {
            if num != first {
                return false;
            }
        }
    }
    true
}

fn calculate_new_number_given_differences(all_sequences: &Vec<Vec<i32>>, next: bool) -> i32 {
    let mut current_final_number = 0;
    for sequence in all_sequences {
        if !next {
            current_final_number = sequence[0] - current_final_number;
        } else {
            current_final_number += sequence.last().unwrap();
        }
    }
    current_final_number
}

impl Solution for Day09 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let part_1 = calculate_numbers(_parsed_input.to_string(), true);
        part_1.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let part_2 = calculate_numbers(_parsed_input.to_string(), false);
        part_2.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day09_both_case1() {
        assert_eq!(Day09::solve("0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45", false), ("114".to_string(), "2".to_string()))
    }
}
