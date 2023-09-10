pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .filter(|s| !s.is_empty())
            .rev()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("the sky is blue");
        let expected = String::from("blue is sky the");

        assert_eq!(Solution::reverse_words(s), expected);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("  hello world  ");
        let expected = String::from("world hello");

        assert_eq!(Solution::reverse_words(s), expected);
    }

    #[test]
    fn test_example_3() {
        let s = String::from("a good   example");
        let expected = String::from("example good a");

        assert_eq!(Solution::reverse_words(s), expected);
    }
}
