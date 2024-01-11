pub struct Solution;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn apply(&self, a: i32, b: i32) -> i32 {
        match self {
            Operation::Add => a + b,
            Operation::Subtract => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
        }
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let function_map: HashMap<&str, Operation> = vec![
            ("+", Operation::Add),
            ("-", Operation::Subtract),
            ("*", Operation::Multiply),
            ("/", Operation::Divide),
        ]
        .into_iter()
        .collect();
        let mut stack: Vec<i32> = vec![];

        for token in tokens {
            match function_map.get(token.as_str()) {
                Some(op) => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(op.apply(a, b));
                }
                None => {
                    let digit = token.parse::<i32>().unwrap();
                    stack.push(digit);
                }
            }
        }

        match stack.pop() {
            Some(res) => res,
            None => panic!["Stack is empty, cannot pop a value"],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_eval_one_operator() {
        let tokens: Vec<String> = vec![String::from("2"), String::from("1"), String::from("+")];
        assert_eq!(Solution::eval_rpn(tokens), 3);
    }

    #[test]
    fn test_can_eval_two_operators() {
        let tokens: Vec<String> = vec![
            String::from("2"),
            String::from("1"),
            String::from("+"),
            String::from("3"),
            String::from("*"),
        ];
        assert_eq!(Solution::eval_rpn(tokens), 9);
    }

    #[test]
    fn test_can_eval_two_operators_in_sequence() {
        let tokens: Vec<String> = vec![
            String::from("4"),
            String::from("13"),
            String::from("5"),
            String::from("/"),
            String::from("+"),
        ];
        assert_eq!(Solution::eval_rpn(tokens), 6);
    }
}
