pub struct Solution;

// THIS IS NOT A STACK PROBLEM. THIS IS A BACKTRACKING PROBLEM. NEETCODE CAN SMD.
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        if n == 1 {
            output.push(String::from("()"));
        } else {
            let tmp = Self::generate_parenthesis(n - 1);
            println!("{:?}", tmp);
        }

        output
    }

    fn string_builder(s: &str, n: i32) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_equals_1() {
        let n = 1;
        let expected_output: Vec<String> = vec![String::from("()")];
        assert_eq!(Solution::generate_parenthesis(n), expected_output)
    }

    #[test]
    fn test_input_equals_2() {
        let n = 2;
        let expected_output: Vec<String> = vec![String::from("(())"), String::from("()()")];
        assert_eq!(Solution::generate_parenthesis(n), expected_output)
    }

    #[test]
    fn test_input_equals_3() {
        let n = 3;
        let expected_output: Vec<String> = vec![
            String::from("((()))"),
            String::from("(()())"),
            String::from("(())()"),
            String::from("()(())"),
            String::from("()()()"),
        ];
        assert_eq!(Solution::generate_parenthesis(n), expected_output)
    }
}
