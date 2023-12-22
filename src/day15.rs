use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day15;

fn calculate_part_1(contents: String) -> usize {
    let mut values = Vec::new();
    for element in contents.split(",") {
        let value = calculate_the_value_post_hash(&element.to_string());
        values.push(value);
    }
    let sum = values.iter().sum();
    sum
}

fn calculate_part_2(contents: String) -> usize {
    let mut box_of_lenses: HashMap<usize, Vec<(String, usize)>> = HashMap::new();
    for element in contents.split(",") {
        if element.contains("-") {
            let element = element.to_string();
            let lens_label = &element[0..&element.len() - 1];
            let box_number = calculate_the_value_post_hash(&lens_label.to_string());
            if let Some(lenses) = box_of_lenses.get_mut(&box_number) {
                lenses.retain(|lens| lens.0 != lens_label);
            }
        } else {
            let parts: Vec<&str> = element.split('=').collect();
            let lens_label = parts[0].to_string();
            let focal_length = parts[1].parse::<usize>().unwrap();
            let box_number = calculate_the_value_post_hash(&lens_label);

            if let Some(lenses) = box_of_lenses.get_mut(&box_number) {
                let new_lens_box = work_out_new_lens_box(lens_label, focal_length, lenses.to_vec());
                box_of_lenses.insert(box_number, new_lens_box);
            } else {
                let mut new_lens_box = Vec::new();
                new_lens_box.push((lens_label, focal_length));
                box_of_lenses.insert(box_number, new_lens_box);
            }
        }
    }
    let result = calculate_focus_power_of_lens_boxes(box_of_lenses);
    result
}

fn calculate_focus_power_of_lens_boxes(
    box_of_lenses: HashMap<usize, Vec<(String, usize)>>,
) -> usize {
    let mut result = 0;
    for (box_number, lenses) in box_of_lenses {
        let box_number_multiplier = box_number + 1;
        let mut box_power = 0;
        let mut lens_slot = 1;
        for lens in lenses {
            box_power += (box_number_multiplier) * (lens_slot) * lens.1;
            lens_slot += 1;
        }
        result += box_power;
    }
    result
}

fn work_out_new_lens_box(
    lens_label: String,
    focal_length: usize,
    lens_box: Vec<(String, usize)>,
) -> Vec<(String, usize)> {
    let mut new_lens_box = Vec::new();
    let mut lens_label_found = false;
    for element in lens_box {
        if element.0 == lens_label {
            new_lens_box.push((lens_label.clone(), focal_length));
            lens_label_found = true;
        } else {
            new_lens_box.push(element)
        }
    }
    if !lens_label_found {
        new_lens_box.push((lens_label, focal_length));
    }
    new_lens_box
}

fn calculate_the_value_post_hash(input: &String) -> usize {
    let mut result = 0;
    for char in input.chars() {
        result += char as usize;
        result *= 17;
        result %= 256;
    }
    result
}

impl Solution for Day15 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let count = calculate_part_1(_parsed_input.to_string());
        count.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let count = calculate_part_2(_parsed_input.to_string());
        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day15_both_case1() {
        assert_eq!(
            Day15::solve("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", false),
            ("1320".to_string(), "145".to_string())
        )
    }
}
