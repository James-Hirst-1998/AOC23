use std::collections::HashMap;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day02;

fn sum_of_full_cube_lines(contents: String) -> i32{
    let full_cube_lines = find_lines_with_possible_cubes(contents);

    println!("{:?}", full_cube_lines);

    let mut sum_of_full_lines = 0;
    for id in full_cube_lines{
        sum_of_full_lines += id;
    }

    sum_of_full_lines
}

fn sum_of_power_set(contents: String) -> i32{
    let full_cube_lines = find_power_set_of_min_cubes(contents);

    println!("{:?}", full_cube_lines);

    let mut sum_of_power_set = 0;
    for id in full_cube_lines{
        sum_of_power_set += id;
    }

    sum_of_power_set
}

// Game 7: 2 green, 9 red, 9 blue; 12 red, 14 blue; 8 red, 3 green

fn find_power_set_of_min_cubes(contents: String) -> Vec<i32> {
    let mut power_set: Vec<i32> = Vec::new();
    
    let lines = contents.split("\n");
    for (index, line) in lines.enumerate() {
        let game_index = index+1;
        let game_string = format!("Game {}:", game_index);
        let line_without_game = line.replace(&game_string, "");

        let max_cubes = find_max_cubes_per_line(line_without_game.to_string());

        let green_max = max_cubes.get("green").unwrap();
        let red_max = max_cubes.get("red").unwrap();
        let blue_max = max_cubes.get("blue").unwrap();

        power_set.push(green_max * red_max * blue_max)
    }
    power_set

}

fn find_lines_with_possible_cubes(contents: String) -> Vec<i32> {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut lines_with_possible_cubes: Vec<i32> = Vec::new();
    
    let lines = contents.split("\n");
    for (index, line) in lines.enumerate() {
        let game_index = index+1;
        let game_string = format!("Game {}:", game_index);
        let line_without_game = line.replace(&game_string, "");

        let max_cubes = find_max_cubes_per_line(line_without_game.to_string());

        let mut line_impossible = false;
        let green_max = max_cubes.get("green").unwrap();
        let red_max = max_cubes.get("red").unwrap();
        let blue_max = max_cubes.get("blue").unwrap();

        if green_max > &max_green {
            line_impossible = true
        }
        if red_max > &max_red {
            line_impossible = true
        }
        if blue_max > &max_blue {
            line_impossible = true
        }

        if !line_impossible {
            lines_with_possible_cubes.push(game_index.try_into().unwrap());
        }

    } 

    lines_with_possible_cubes
}

fn find_max_cubes_per_line(line: String) -> HashMap<&'static str, i32>{
    let mut max_cubes: HashMap<&str, i32> = HashMap::new();
    let mut greens_found: Vec<i32> = Vec::new();
    let mut reds_found: Vec<i32> = Vec::new();
    let mut blues_found: Vec<i32> = Vec::new();

    let hand_pulls = line.split(";");
    for hand in hand_pulls {
        let colour_splits = hand.split(",");
        for colour in colour_splits {
            let mut words = colour.split_whitespace();
            let number: i32 = words.next().unwrap().parse().unwrap();

            if colour.contains("green"){
                greens_found.push(number);
            }
            if colour.contains("red"){
                reds_found.push(number);
            }
            if colour.contains("blue"){
                blues_found.push(number);
            }

        }
    }

    let max_green = greens_found.iter().max().unwrap();
    let max_red = reds_found.iter().max().unwrap();
    let max_blue = blues_found.iter().max().unwrap();

    max_cubes.insert("green", *max_green);
    max_cubes.insert("red", *max_red);
    max_cubes.insert("blue", *max_blue);
    max_cubes
}

impl Solution for Day02 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let sum_of_full_lines = sum_of_full_cube_lines(_parsed_input.to_string());
        sum_of_full_lines.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let sum_of_power_set = sum_of_power_set(_parsed_input.to_string());
        sum_of_power_set.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(Day02::solve("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", false), ("8".to_string(), "2286".to_string()))
    }
}
