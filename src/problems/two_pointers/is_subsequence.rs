pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t_iter = t.chars();
        for s_char in s.chars() {
            if t_iter.find(|&t_char| t_char == s_char).is_none() {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");
        let expected = true;

        assert_eq!(Solution::is_subsequence(s, t), expected);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");
        let expected = false;

        assert_eq!(Solution::is_subsequence(s, t), expected);
    }
}
