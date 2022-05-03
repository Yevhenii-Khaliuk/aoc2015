use std::collections::HashMap;

use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! { static ref DIGITS: Regex = Regex::new(r"\d+").unwrap(); }

pub fn puzzle1(input: &str, key: &str) -> u16 {
    let mut circuit = Circuit::new(input);
    circuit.evaluate(key)
}

pub fn puzzle2(_input: &str) -> u16 {
    0
}

struct Circuit<'a> {
    instructions: HashMap<&'a str, Operator<'a>>,
}

impl<'a> Circuit<'a> {
    fn new(input: &'a str) -> Self {
        Circuit {
            instructions: Circuit::parse_instructions(input),
        }
    }

    fn parse_instructions(input: &str) -> HashMap<&str, Operator> {
        let mut instructions = HashMap::new();
        for line in input.lines() {
            let line_split = line.split(" -> ").collect::<Vec<&str>>();
            let operation = line_split[0];
            let op_split = operation.split_whitespace().collect::<Vec<&str>>();
            let operation = match op_split.len() {
                3 => Operator::with_two_operands(op_split[1], op_split[0], op_split[2]),
                2 => Operator::Not(op_split[1]),
                1 if DIGITS.is_match(op_split[0]).unwrap() => Operator::Evaluated(op_split[0].parse().unwrap()),
                1 => Operator::Value(op_split[0]),
                _ => unreachable!(),
            };
            instructions.insert(line_split[1], operation);
        }
        instructions
    }

    fn evaluate(&mut self, key: &str) -> u16 {
        let operator = self.instructions.get(key).unwrap();
        match operator {
            Operator::And { left, right } => {
                let left = *left;
                let right = *right;
                self.process_two(left, right, |x, y| x & y)
            }
            Operator::Or { left, right } => {
                let left = *left;
                let right = *right;
                self.process_two(left, right, |x, y| x | y)
            }
            Operator::Lshift { left, right } => {
                let left = *left;
                let right = *right;
                self.process_two(left, right, |x, y| x << y)
            }
            Operator::Rshift { left, right } => {
                let left = *left;
                let right = *right;
                self.process_two(left, right, |x, y| x >> y)
            }
            Operator::Not(operand) => {
                let operand = *operand;
                !self.parse_digit(operand)
            }
            Operator::Value(operand) => {
                let operand = *operand;
                self.parse_digit(operand)
            }
            Operator::Evaluated(operand) => *operand,
        }
    }

    fn process_two(&mut self, left: &'a str, right: &'a str, processor: fn(u16, u16) -> u16) -> u16 {
        let left = self.parse_digit(left);
        let right = self.parse_digit(right);
        processor(left, right)
    }

    fn parse_digit(&mut self, operand: &'a str) -> u16 {
        if DIGITS.is_match(operand).unwrap() {
            operand.parse().unwrap()
        } else {
            let value = self.evaluate(operand);
            self.instructions.insert(operand, Operator::Evaluated(value));
            value
        }
    }
}

enum Operator<'a> {
    And { left: &'a str, right: &'a str },
    Or { left: &'a str, right: &'a str },
    Lshift { left: &'a str, right: &'a str },
    Rshift { left: &'a str, right: &'a str },
    Not(&'a str),
    Value(&'a str),
    Evaluated(u16),
}

impl<'a> Operator<'a> {
    fn with_two_operands(s: &'a str, left: &'a str, right: &'a str) -> Self {
        match s {
            "AND" => Operator::And { left, right },
            "OR" => Operator::Or { left, right },
            "LSHIFT" => Operator::Lshift { left, right },
            "RSHIFT" => Operator::Rshift { left, right },
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day7::{puzzle1, puzzle2};

    #[test]
    fn test_puzzle1() {
        let input = "123 -> x\n\
            456 -> y\n\
            x AND y -> d\n\
            x OR y -> e\n\
            x LSHIFT 2 -> f\n\
            y RSHIFT 2 -> g\n\
            NOT x -> h\n\
            NOT y -> i";
        assert_eq!(72, puzzle1(input, "d"));
        assert_eq!(507, puzzle1(input, "e"));
        assert_eq!(492, puzzle1(input, "f"));
        assert_eq!(114, puzzle1(input, "g"));
        assert_eq!(65412, puzzle1(input, "h"));
        assert_eq!(65079, puzzle1(input, "i"));
        assert_eq!(123, puzzle1(input, "x"));
        assert_eq!(456, puzzle1(input, "y"));
    }

    #[test]
    fn test_puzzle2() {
        assert_eq!(0, puzzle2(""));
    }
}
