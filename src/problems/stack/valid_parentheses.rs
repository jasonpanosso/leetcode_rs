pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];
        for ch in s.chars() {
            match ch {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}' | ')' | ']' if Some(ch) != stack.pop() => return false,
                _ => (),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_bracket_types_expect_true() {
        let s = String::from("()[]{}");
        assert_eq!(Solution::is_valid(s), true);
    }

    #[test]
    fn test_single_parenthesis_expect_true() {
        let s = String::from("()");
        assert_eq!(Solution::is_valid(s), true);
    }

    #[test]
    fn test_incorrect_match_expect_false() {
        let s = String::from("(]");
        assert_eq!(Solution::is_valid(s), false);
    }
    
    #[test]
    fn test_can_handle_popping_from_empty_stack() {
        let s = String::from("}");
        assert_eq!(Solution::is_valid(s), false);
    }
}
