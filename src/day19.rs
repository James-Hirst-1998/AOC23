use std::{collections::HashMap, time::Instant};

use regex::Regex;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day19;

#[derive(Debug)]
pub struct MachineParts {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl MachineParts {
    fn total_value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
pub struct Rule {
    character: char,
    comparison: char,
    value: usize,
    outcome: String,
}

impl Rule {
    fn find_outcome(&self, machine_part: &MachineParts) -> String {
        let character_in_question = match self.character {
            'x' => machine_part.x,
            'm' => machine_part.m,
            'a' => machine_part.a,
            's' => machine_part.s,
            _ => panic!("Invalid character"),
        };
        let rule_statisfied = match self.comparison {
            '<' => character_in_question < self.value,
            '>' => character_in_question > self.value,
            _ => panic!("Invalid comparison"),
        };
        if rule_statisfied {
            return self.outcome.clone();
        } else {
            return String::new();
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    rules: Vec<Rule>,
    final_operation: String,
}

impl Workflow {
    fn find_outcome(&self, machine_part: &MachineParts) -> String {
        let mut outcome = String::new();
        for rule in &self.rules {
            let rule_outcome = rule.find_outcome(machine_part);
            if rule_outcome != "" {
                outcome = rule_outcome;
                break;
            }
        }
        if outcome == "" {
            outcome = self.final_operation.clone();
        }
        outcome
    }
}

fn calculate_part_1(contents: String) -> usize {
    let (workflows, machine_parts) = parse_input(contents);
    let mut accepted_parts = Vec::new();
    let start = Instant::now();
    for machine_part in machine_parts {
        let accepted = find_if_machine_part_accepted(&workflows, &machine_part);
        if accepted {
            accepted_parts.push(machine_part);
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    accepted_parts.iter().map(|x| x.total_value()).sum()
}

// fn calculate_part_2(contents: String) -> usize {
//     let (workflows, _machine_parts) = parse_input(contents);
//     let comparison_points = find_all_comparison_points(&workflows);
//     println!("{:?}", comparison_points);
//     0
// }

// fn create_all_machine_parts_from_comparison_points(comparison_points: HashMap<String, Vec<usize>>) -> Vec<Workflow> {
//     let possible_machine_parts = Vec::new();

// }

// fn find_all_comparison_points(workflows: &HashMap<String, Workflow>) -> HashMap<String, Vec<usize>> {
//     let mut comparison_points = HashMap::new();
//     for (_name, workflow) in workflows {
//         for rule in &workflow.rules {
//             let character = rule.character.to_string();
//             let value = rule.value;
//             comparison_points
//                 .entry(character)
//                 .or_insert(Vec::new())
//                 .push(value);
//         }
//     }

//     // Sort the values and remove duplicates
//     for (_character, values) in &mut comparison_points {
//         values.push(0);
//         values.push(4000);
//         values.sort();
//         values.dedup();
//     }
//     comparison_points

// }

fn find_if_machine_part_accepted(
    workflows: &HashMap<String, Workflow>,
    machine_part: &MachineParts,
) -> bool {
    let mut current_location = "in".to_string();
    loop {
        let workflow = workflows.get(&current_location).unwrap();
        let outcome = workflow.find_outcome(&machine_part);
        if outcome == "A" {
            return true;
        } else if outcome == "R" {
            return false;
        } else {
            current_location = outcome;
        }
    }
}

fn parse_input(contents: String) -> (HashMap<String, Workflow>, Vec<MachineParts>) {
    let mut machine_parts = Vec::new();
    let mut workflows = HashMap::new();

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        } else if line.starts_with('{') {
            let machine_part = parse_machine_part_line(line.to_string());
            machine_parts.push(machine_part);
        } else {
            let (name, workflow) = parse_workflow_line(line.to_string());
            workflows.insert(name, workflow);
        }
    }
    (workflows, machine_parts)
}

fn parse_workflow_line(line: String) -> (String, Workflow) {
    // For a line like px{a<2006:qkq,m>2090:A,rfg} I want to string the first
    // bit off and set name=px. Then for each comma separated rule I want to convert
    // it into a rule struct with the character, comparison, value and outcome.
    let mut rules = Vec::new();
    let mut final_operation = String::new();

    let re = Regex::new(r"(?P<name>.*?)\{(?P<rules>.*)\}").unwrap();
    let caps = re.captures(&line).unwrap();
    let name = caps.name("name").unwrap().as_str().to_string();
    let rules_string = caps.name("rules").unwrap().as_str().to_string();

    let rules_split = rules_string.split(",");
    for rule in rules_split {
        if rule.contains(":") {
            let rule_split = rule.split(":");
            let rule_vec: Vec<&str> = rule_split.collect();
            let rule = Rule {
                character: rule_vec[0].chars().next().unwrap(),
                comparison: rule_vec[0].chars().nth(1).unwrap(),
                value: rule_vec[0][2..].parse::<usize>().unwrap(),
                outcome: rule_vec[1].to_string(),
            };
            rules.push(rule);
        } else {
            final_operation = rule.to_string();
        }
    }
    (
        name,
        Workflow {
            rules,
            final_operation,
        },
    )
}

fn parse_machine_part_line(line: String) -> MachineParts {
    let re = Regex::new(r"x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)").unwrap();
    let caps = re.captures(&line).unwrap();
    let x = caps.name("x").unwrap().as_str().parse::<usize>().unwrap();
    let m = caps.name("m").unwrap().as_str().parse::<usize>().unwrap();
    let a = caps.name("a").unwrap().as_str().parse::<usize>().unwrap();
    let s = caps.name("s").unwrap().as_str().parse::<usize>().unwrap();

    MachineParts { x, m, a, s }
}

impl Solution for Day19 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(_parsed_input: &mut Self::ParsedInput) -> String {
        let count = calculate_part_1(_parsed_input.to_string());
        count.to_string()
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
    fn check_day19_part1_case1() {
        assert_eq!(
            Day19::solve_part_one(
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            ),
            "19114".to_string()
        )
    }

    // #[test]
    // fn check_day19_part2_case1() {
    //     assert_eq!(Day19::solve_part_two(""), "0".to_string())
    // }

    // #[test]
    // fn check_day19_both_case1() {
    //     assert_eq!(Day19::solve("", false), ("0".to_string(), "0".to_string()))
    // }
}
