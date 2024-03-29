use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day23;

impl Solution for Day23 {
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
    fn check_day23_part1_case1() {
        assert_eq!(Day23::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day23_part2_case1() {
        assert_eq!(Day23::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day23_both_case1() {
        assert_eq!(Day23::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
