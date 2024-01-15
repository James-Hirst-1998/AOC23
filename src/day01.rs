use crate::Solution;
use std::collections::HashMap;

// I originally did this in python so not included in this repo

#[derive(Clone, Debug)]
pub struct Day01;

fn get_conversions() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("one", "o1e");
    m.insert("two", "t2o");
    m.insert("three", "t3h");
    m.insert("four", "f4o");
    m.insert("five", "f5i");
    m.insert("six", "s6i");
    m.insert("seven", "s7e");
    m.insert("eight", "e8i");
    m.insert("nine", "n9e");
    m.insert("zero", "z0o");
    m
}

fn find_all_numbers_in_line(line: String) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for character in line.chars() {
        if character.is_digit(10){
            numbers.push(character.to_digit(10).unwrap());
        }
    }
    numbers
}

fn find_number_from_line(line: String) -> u32 {
    let numbers_in_line = find_all_numbers_in_line(line);
    // Join the first and last element on the vector to form a 2 digit number
    numbers_in_line[0] * 10 + numbers_in_line[numbers_in_line.len() - 1]
}

fn find_numbers_from_all_lines(lines: String) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for line in lines.lines() {
        numbers.push(find_number_from_line(line.to_string()));
    }
    numbers
}

fn find_sum_of_numbers(numbers: Vec<u32>) -> u32 {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }
    sum
}

fn replace_string_numbers(input_string: String) -> String {
    let conversions = get_conversions();
    let mut output_string = input_string.clone();
    for (key, value) in conversions {
        output_string = output_string.replace(key, value);
    }
    output_string
}

impl Solution for Day01 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let numbers = find_numbers_from_all_lines(_parsed_input.to_string());
        let sum = find_sum_of_numbers(numbers);
        sum.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let replaced_string = replace_string_numbers(_parsed_input.to_string());
        let numbers = find_numbers_from_all_lines(replaced_string);
        let sum = find_sum_of_numbers(numbers);
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(Day01::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(Day01::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(Day01::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
