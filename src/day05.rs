use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day05;

pub struct Map {
    mapping_elements: Vec<MappingElement>,
}

impl Map {
    fn new() -> Map {
        Map {
            mapping_elements: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.mapping_elements.is_empty()
    }

    fn add_element(&mut self, line: String) {
        let values: Vec<&str> = line.split_whitespace().collect();
        let mapping_value: usize = values[0].parse::<usize>().unwrap();
        let start_value = values[1].parse::<usize>().unwrap();
        let range = values[2].parse::<usize>().unwrap();

        self.mapping_elements.push(MappingElement {
            start_value,
            mapping_value,
            range,
        });
    }

    fn calculate_end_position(&self, value: usize) -> usize {
        for element in &self.mapping_elements {
            if element.in_range(value) {
                return element.calculate_end_position(value);
            }
        }
        return value;
    }
}

#[derive(Debug)]
pub struct MappingElement {
    start_value: usize,
    mapping_value: usize,
    range: usize,
}

impl MappingElement {
    fn in_range(&self, value: usize) -> bool {
        value >= self.start_value && value <= self.start_value + self.range
    }

    fn calculate_end_position(&self, value: usize) -> usize {
        self.mapping_value + value - self.start_value
    }
}

fn calculate_seed_positions_part_1(contents: String) -> Vec<usize> {
    let first_line = contents.lines().next().unwrap();
    let first_line_stripped = first_line.split_once(":").unwrap().1;
    let vec_of_maps = parse_lines_to_maps(contents.clone());

    let seeds: Vec<&str> = first_line_stripped.split_whitespace().collect();
    let owned_seeds: Vec<usize> = seeds.iter().map(|&s| s.parse::<usize>().unwrap()).collect();

    find_seed_end_positions(owned_seeds, vec_of_maps)
}

// fn calculate_seed_positions_part_2(contents: String) -> Vec<usize> {
//     let first_line = contents.lines().next().unwrap();
//     let first_line_stripped = first_line.split_once(":").unwrap().1;
//     let vec_of_maps = parse_lines_to_maps(contents.clone());

//     let seeds: Vec<&str> = first_line_stripped.split_whitespace().collect();
//     let mut owned_seeds: Vec<usize> = seeds.iter().map(|&s| s.parse::<usize>().unwrap()).collect();

//     owned_seeds = calculate_seed_input_for_part_2(owned_seeds, 4 as u32);
//     find_seed_end_positions(owned_seeds, vec_of_maps)
// }

// fn calculate_seed_input_for_part_2(seeds: Vec<usize>, starting_position: u32) -> Vec<usize> {
//     let mut updated_seed_input: Vec<usize> = Vec::new();
//     let starting_seed: usize = seeds.get(starting_position as usize).unwrap().clone();
//     let range = seeds.get(starting_position as usize).unwrap().clone();
//     for i in 0..range {
//         updated_seed_input.push(starting_seed + i);
//     }
//     updated_seed_input
// }

fn find_seed_end_positions(owned_seeds: Vec<usize>, vec_of_maps: Vec<Map>) -> Vec<usize> {
    let mut end_seed_positions: Vec<usize> = Vec::new();
    for seed in owned_seeds {
        let seed = seed;
        let mut current_seed = seed;
        for map in &vec_of_maps {
            current_seed = map.calculate_end_position(current_seed);
        }
        end_seed_positions.push(current_seed);
    }
    end_seed_positions
}

fn parse_lines_to_maps(contents: String) -> Vec<Map> {
    let mut vec_of_maps: Vec<Map> = Vec::new();
    let mut current_map = Map::new();
    for line in contents.lines().skip(1) {
        if line.contains("map") {
            if !current_map.is_empty() {
                vec_of_maps.push(current_map);
            }
            current_map = Map::new();
            continue;
        } else if line.is_empty() {
            continue;
        } else {
            current_map.add_element(line.to_string());
        }
    }
    vec_of_maps.push(current_map);
    vec_of_maps
}

impl Solution for Day05 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // Change the return type of this function by editing the ParsedInput type above.
        // You can skip this and pass the raw string to each part.
        // Alternatively, you can parse the input here, either working on the same mutable struct
        // in parts one and two or passing a tuple with the data required for each part.
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let seed_end_positions = calculate_seed_positions_part_1(_parsed_input.to_string());
        let lowest_seed = seed_end_positions.iter().min().unwrap();
        lowest_seed.to_string()
    }

    fn part_two(_parsed_input: &mut Self::ParsedInput) -> String {
        // Very slow so commented out for now
        // let seed_end_positions = calculate_seed_positions_part_2(_parsed_input.to_string()); 
        // let lowest_seed = seed_end_positions.iter().min().unwrap();
        // lowest_seed.to_string()
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // This test is currently broken
    fn check_day05_both_case1() {
        assert_eq!(
            Day05::solve(
                "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4",
                false
            ),
            ("35".to_string(), "46".to_string())
        )
    }
}
