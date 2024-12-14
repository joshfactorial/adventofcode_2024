use std::fs::File;
use std::io;
use std::ops::{Add, Mul};
use std::io::{BufRead, BufReader, Write};

struct Equation {
    lhs: u64,
    rhs: Vec<u64>,
}

impl Equation {
    fn is_valid(&self, extended_operator_set: bool) -> bool {
        let last_element = self.rhs.last().unwrap();
        if self.rhs.len() == 1 {
            return if *last_element == self.lhs {
                true
            } else {
                false
            };
        };

        if extended_operator_set {
            let lhs_string = self.lhs.to_string();
            let last_element_string = last_element.to_string();
            let can_be_concatenated = lhs_string.ends_with(&last_element_string);
            if can_be_concatenated {
                let mut new_rhs = self.rhs.clone();
                new_rhs.pop();

                let new_equation = Self {
                    lhs: lhs_string
                        .strip_suffix(&last_element_string)
                        .unwrap()
                        .parse::<u64>()
                        .unwrap_or(0),
                    rhs: new_rhs,
                };

                if new_equation.is_valid(true) {
                    return true
                }
            }
        }
        let is_lhs_divisible_by_last_element = self.lhs % last_element == 0;
        if is_lhs_divisible_by_last_element {
            let mut new_rhs = self.rhs.clone();
            new_rhs.pop();

            let new_equation = Self {
                lhs: self.lhs / last_element,
                rhs: new_rhs,
            };
            if new_equation.is_valid(extended_operator_set) {
                return true;
            }
        }

        let mut new_rhs = self.rhs.clone();
        new_rhs.pop();

        let new_equation = Self {
            lhs: self.lhs.saturating_sub(*last_element),
            rhs: new_rhs,
        };

        if new_equation.is_valid(extended_operator_set) {
            return true;
        }

        false
    }
}

pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    // This creates a buffer to read lines
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    Some(
        equations
            .iter()
            .filter_map(|equation| match equation.is_valid(false) {
                true => Some(equation.lhs),
                false => None,
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    Some(
        equations
            .iter()
            .filter_map(|equation| match equation.is_valid(true) {
                true => Some(equation.lhs),
                false => None,
            })
            .sum()
    )
}

fn parse_input(filename: &str) -> Vec<Equation> {
    let file = File::open(filename).unwrap();
    let input = BufReader::new(file).lines();
    input
        .map(|l| {
            let l_unwrapped = l.unwrap();
            let mut parts = l_unwrapped.split(":");
            let lhs = parts.next().unwrap().parse::<u64>().unwrap();
            let rhs = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|v| v.parse::<u64>().unwrap())
                .collect();

            Equation { lhs, rhs }
        })
        .collect()
}



fn main() {
    // let result = part_one("/home/joshfactorial/code/inputs/inputs.txt.full").unwrap();
    let result = part_two("/home/joshfactorial/code/inputs/inputs.txt.full").unwrap();
    println!("Result = {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup() {
        let input = parse_input("/home/joshfactorial/code/inputs/inputs.txt");
        assert_eq!(Vec::from([10, 19]), input[&190]);
        assert_eq!(Vec::from([11, 6, 16, 20]), input[&292]);
    }
}