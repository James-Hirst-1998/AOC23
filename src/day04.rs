use std::collections::HashMap;

use crate::Solution;

fn sum_lines(contents: String) -> i32 {
    let full_lines = find_points_per_card(contents);

    let mut sum_of_full_lines = 0;
    for id in full_lines {
        sum_of_full_lines += id;
    }

    sum_of_full_lines
}

fn sum_scratchcards(contents: String) -> u32 {
    let scratchcards: HashMap<u32, HashMap<&str, u32>> =
        find_num_of_each_scratchcard(contents);

    let mut sum_of_scratchcards = 0;
    for (_card, values) in scratchcards {
        if let Some(num_of_cards) = values.get("Num_of_cards") {
            sum_of_scratchcards += num_of_cards;
        }
    }
    sum_of_scratchcards
}

fn find_num_of_each_scratchcard(contents: String) -> HashMap<u32, HashMap<&'static str, u32>> {
    let mut scratchcards: HashMap<u32, HashMap<&'static str, u32>> = HashMap::new();

    let lines = contents.split("\n");
    for (index, line) in lines.enumerate() {
        let card_index: u32 = (index + 1).try_into().unwrap_or(1);
        let line_without_card = line.split(":").last().unwrap_or("");

        let winning_numbers: Vec<i32> = find_winning_numbers_in_card(line_without_card.to_string());
        let mut inner_map: HashMap<&'static str, u32> = HashMap::new();
        inner_map.insert("Num_of_cards", 1);
        inner_map.insert("Num_of_points", winning_numbers.len().try_into().unwrap());
        scratchcards.insert(card_index, inner_map);
    }

    let number_of_cards = scratchcards.len();
    for card in 1..number_of_cards {
        let card = card.try_into().unwrap_or(1);
        let num_of_current_card: u32 = scratchcards
            .get(&card)
            .unwrap_or(&HashMap::new())
            .get("Num_of_cards")
            .unwrap_or(&1)
            .clone();
        let num_of_pts_of_current_card = scratchcards
            .get(&card)
            .unwrap_or(&HashMap::new())
            .get("Num_of_points")
            .unwrap_or(&0)
            .clone();

        if num_of_pts_of_current_card > 0 {
            let starting_card = card + 1;
            let finishing_card = card + num_of_pts_of_current_card + 1;

            for number in starting_card..finishing_card {
                if let Some(value) = scratchcards.get_mut(&number) {
                    if let Some(num_of_cards) = value.get_mut("Num_of_cards") {
                        *num_of_cards += num_of_current_card;
                    }
                }
            }
        }
    }
    scratchcards
}

fn find_points_per_card(contents: String) -> Vec<i32> {
    let mut points_per_card: Vec<i32> = Vec::new();

    let lines = contents.split("\n");
    for line in lines {
        let line_without_card = line.split(":").last().unwrap_or("");

        let winning_numbers: Vec<i32> = find_winning_numbers_in_card(line_without_card.to_string());
        if winning_numbers.len() > 0 {
            let power: u32 = winning_numbers.len().try_into().unwrap_or(1) - 1;
            let base: i32 = 2;
            points_per_card.push(base.pow(power))
        } else {
            points_per_card.push(0);
        }
    }

    points_per_card
}

fn find_winning_numbers_in_card(card: String) -> Vec<i32> {
    let mut winning_numbers_in_card: Vec<i32> = Vec::new();
    let mut split_card = card.split("|");
    let mut winning_numbers: Vec<i32> = Vec::new();
    let mut elfs_numbers: Vec<i32> = Vec::new();

    if let Some(first_half) = split_card.next() {
        winning_numbers = convert_string_to_integer_list(first_half.to_string());
    }
    if let Some(second_half) = split_card.next() {
        elfs_numbers = convert_string_to_integer_list(second_half.to_string());
    }

    for number in elfs_numbers {
        if winning_numbers.contains(&number) {
            winning_numbers_in_card.push(number)
        }
    }
    winning_numbers_in_card
}

fn convert_string_to_integer_list(input_string: String) -> Vec<i32> {
    let integer_list = input_string
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    integer_list
}

#[derive(Clone, Debug)]
pub struct Day04;

impl Solution for Day04 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let sum_of_full_lines = sum_lines(_parsed_input.to_string());
        sum_of_full_lines.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let sum_of_scratchcards = sum_scratchcards(_parsed_input.to_string());
        sum_of_scratchcards.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(
            Day04::solve(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
                false
            ),
            ("13".to_string(), "30".to_string())
        )
    }
}
