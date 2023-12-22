use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day14;

fn calculate_part_1(contents: String) -> usize {
    let content_input: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut content_swapped = content_input.clone();
    for _ in 0..3 {
        content_swapped = rotate_clockwise(&content_swapped);
    }
    let rocks_moved_left: Vec<String> = move_rocks_left(&content_swapped);
    let total_load = rotate_and_calculate_load(&rocks_moved_left);
    total_load
}

fn calculate_part_2(contents: String, number_of_cycles: usize) -> usize {
    let content_input: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut content_grid = content_input.clone();
    for _ in 0..3 {
        content_grid = rotate_clockwise(&content_grid);
    }

    let mut cycles_completed = 0;
    let mut load_vec = Vec::new();
    let mut loop_found = false;
    while !loop_found {
        content_grid = complete_a_cycle(&content_grid);
        let load = rotate_and_calculate_load(&content_grid);
        // artibitrary number to ensure we are in a repeating loop
        // and out of the start up phase
        if cycles_completed > 200 {
            load_vec.push(load);
        }
        cycles_completed += 1;

        if full_loop_exists(&load_vec) {
            loop_found = true;
        }
    }
    let final_load_after_mod = calculate_final_load(&load_vec, number_of_cycles, cycles_completed);
    final_load_after_mod
}

fn calculate_final_load(
    load_vec: &Vec<usize>,
    number_of_cycles: usize,
    cycles_completed: usize,
) -> usize {
    let length_of_vec = load_vec.len();
    let half_length = length_of_vec / 2;
    let remainder_after_cycles = (number_of_cycles - cycles_completed) % half_length;
    let final_load = load_vec[remainder_after_cycles - 1];
    final_load
}

fn full_loop_exists(load_vec: &Vec<usize>) -> bool {
    let length_of_vec = load_vec.len();
    if length_of_vec == 0 {
        return false;
    }
    let mod_length = length_of_vec % 2;
    if mod_length != 0 {
        return false;
    }
    let half_length = length_of_vec / 2;
    let first_half = &load_vec[0..half_length];
    let second_half = &load_vec[half_length..length_of_vec];
    if first_half == second_half {
        return true;
    }
    return false;
}

fn complete_a_cycle(grid: &Vec<String>) -> Vec<String> {
    let mut grid_after_cycle: Vec<String> = grid.clone();
    for _ in 0..4 {
        grid_after_cycle = move_rocks_left(&grid_after_cycle);
        grid_after_cycle = rotate_clockwise(&grid_after_cycle);
    }

    grid_after_cycle
}

fn rotate_and_calculate_load(rocks_moved: &Vec<String>) -> usize {
    let after_final_rotation = rotate_clockwise(&rocks_moved);
    let numbers_per_line: Vec<usize> = get_number_of_rocks_each_row(&after_final_rotation);
    let number_of_lines = numbers_per_line.len();
    let mut total_load = 0;
    for (index, number) in numbers_per_line.iter().enumerate() {
        let load = number * (number_of_lines - index);
        total_load += load;
    }
    total_load
}

fn get_number_of_rocks_each_row(rocks_in_rows: &Vec<String>) -> Vec<usize> {
    let mut numbers_per_line: Vec<usize> = Vec::new();
    for line in rocks_in_rows {
        let mut number_of_rocks = 0;
        for element in line.chars() {
            if element == 'O' {
                number_of_rocks += 1;
            }
        }
        numbers_per_line.push(number_of_rocks);
    }
    numbers_per_line
}

fn move_rocks_left(input: &Vec<String>) -> Vec<String> {
    let mut rocks_moved_left_rows: Vec<String> = Vec::new();

    for line in input {
        let mut rocks_moved_left_row = String::new();
        let mut space_between_rocks = String::new();
        for character in line.chars() {
            if character != '#' {
                space_between_rocks.push(character);
            } else {
                rocks_moved_left_row.push_str(&move_rocks_in_space_left(&space_between_rocks));
                rocks_moved_left_row.push('#');
                space_between_rocks = String::new();
            }
        }
        rocks_moved_left_row.push_str(&move_rocks_in_space_left(&space_between_rocks));
        rocks_moved_left_rows.push(rocks_moved_left_row);
    }
    rocks_moved_left_rows
}

fn move_rocks_in_space_left(section_of_col: &String) -> String {
    let mut rocks_in_space_moved_left = String::new();
    let length_of_section = section_of_col.len();
    let mut number_of_rocks = 0;
    for character in section_of_col.chars() {
        if character == 'O' {
            number_of_rocks += 1;
        }
    }
    for _ in 0..number_of_rocks {
        rocks_in_space_moved_left.push('O');
    }
    for _ in 0..(length_of_section - number_of_rocks) {
        rocks_in_space_moved_left.push('.');
    }
    rocks_in_space_moved_left
}

fn rotate_clockwise(input: &Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for line in input.iter().rev() {
        for (index, character) in line.chars().enumerate() {
            if output.get(index).is_none() {
                output.push(String::new());
            }
            output[index].push(character);
        }
    }
    output
}

impl Solution for Day14 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let part_1 = calculate_part_1(_parsed_input.to_string());
        part_1.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        let part_2 = calculate_part_2(_parsed_input.to_string(), 1000000000);
        part_2.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day14_both_case1() {
        assert_eq!(
            Day14::solve(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
                false
            ),
            ("136".to_string(), "64".to_string())
        )
    }
}
