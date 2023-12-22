use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day06;

fn multiply_possibilities(contents: String, multiple_races: bool) -> usize {
    let winning_distances_per_race: Vec<Vec<usize>>;

    if multiple_races {
        winning_distances_per_race = find_winning_distances_per_race(contents);
    } else {
        winning_distances_per_race = find_winning_distances_if_one_race(contents);
    }

    let mut multiplication_of_races = 1;
    for race in winning_distances_per_race {
        multiplication_of_races *= race.len();
    }

    multiplication_of_races
}

fn find_winning_distances_per_race(contents: String) -> Vec<Vec<usize>> {
    let mut winning_distances_per_race = Vec::new();
    let mut time_container: HashMap<usize, usize> = HashMap::new();
    let mut distance_record_container: HashMap<usize, usize> = HashMap::new();

    let lines = contents.split("\n");
    let mut line_number = 1;
    for line in lines {
        let line_without_prefix = line.split(":").last().unwrap_or("");
        let line_split_per_race = line_without_prefix.split_whitespace();
        let mut race_number = 1;
        for race in line_split_per_race {
            if line_number == 1 {
                let time: usize = race.parse().expect("Expected a positive integer");
                time_container.insert(race_number, time);
            } else {
                let distance_record = race.parse().expect("Expected a positive integer");
                distance_record_container.insert(race_number, distance_record);
            }
            race_number += 1
        }
        line_number += 1
    }

    let number_of_races_range = time_container.len() + 1;
    for race in 1..number_of_races_range {
        let winning_distances = find_winning_distances_for_a_race(
            time_container.get(&(race as usize)).unwrap(),
            distance_record_container.get(&(race as usize)).unwrap(),
        );
        winning_distances_per_race.push(winning_distances);
    }

    winning_distances_per_race
}

fn find_winning_distances_if_one_race(contents: String) -> Vec<Vec<usize>> {
    let mut winning_distances_per_race = Vec::new();
    let mut time_container: HashMap<usize, usize> = HashMap::new();
    let mut distance_record_container: HashMap<usize, usize> = HashMap::new();

    let lines = contents.split("\n");
    let mut line_number = 1;
    for line in lines {
        let line_without_prefix = line.split(":").last().unwrap_or("");
        let line_without_spaces = line_without_prefix.replace(" ", "");
        let race_number = 1;

        if line_number == 1 {
            let time: usize = line_without_spaces
                .parse()
                .expect("Expected a positive integer");
            time_container.insert(race_number, time);
        } else {
            let distance_record: usize = line_without_spaces
                .parse()
                .expect("Expected a positive integer");
            distance_record_container.insert(race_number, distance_record);
        }

        line_number += 1
    }

    let number_of_races_range = time_container.len() + 1;
    for race in 1..number_of_races_range {
        let winning_distances = find_winning_distances_for_a_race(
            time_container.get(&race).unwrap(),
            distance_record_container.get(&race).unwrap(),
        );
        winning_distances_per_race.push(winning_distances);
    }

    winning_distances_per_race
}

fn find_winning_distances_for_a_race(time: &usize, winning_distance: &usize) -> Vec<usize> {
    let mut winning_distances = Vec::new();
    for winding_time in 0..time + 1 {
        let distance = winding_time * (time - winding_time);
        if distance > winning_distance.clone() {
            winning_distances.push(distance);
        }
    }
    winning_distances
}

impl Solution for Day06 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let multiple_possibilities = multiply_possibilities(_parsed_input.to_string(), true);
        multiple_possibilities.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let multiple_possibilities = multiply_possibilities(_parsed_input.to_string(), false);
        multiple_possibilities.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(
            Day06::solve(
                "Time:      7  15   30
        Distance:  9  40  200",
                false
            ),
            ("288".to_string(), "71503".to_string())
        )
    }
}
