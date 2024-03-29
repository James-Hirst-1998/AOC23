use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day24;

impl Solution for Day24 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        // TODO: implement part one
        0.to_string()
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
    fn check_day24_part1_case1() {
        assert_eq!(Day24::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day24_part2_case1() {
        assert_eq!(Day24::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day24_both_case1() {
        assert_eq!(Day24::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
