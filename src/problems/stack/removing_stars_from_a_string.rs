pub struct Solution;

impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack: Vec<char> = vec![];
        for char in s.chars() {
            if char == '*' {
                stack.pop();
            } else {
                stack.push(char)
            }
        }

        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("leet**cod*e");
        let expected = String::from("lecoe");

        assert_eq!(Solution::remove_stars(s), expected);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("erase*****");
        let expected = String::from("");

        assert_eq!(Solution::remove_stars(s), expected);
    }
}
