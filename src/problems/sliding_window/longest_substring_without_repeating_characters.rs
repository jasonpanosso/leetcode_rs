pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut longest_seq = 0;
        let mut left = 0;

        for (right, c) in s.char_indices() {
            if let Some(prev_index) = map.insert(c, right) {
                if prev_index >= left {
                    longest_seq = longest_seq.max(right - left);
                    left = prev_index + 1;
                }
            }
        }
        longest_seq.max(s.len() - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_1() {
        let s = String::from("abcabcbb");
        assert_eq!(Solution::length_of_longest_substring(s), 3)
    }

    #[test]
    fn test_group_2() {
        let s = String::from("bbbbb");
        assert_eq!(Solution::length_of_longest_substring(s), 1)
    }

    #[test]
    fn test_group_3() {
        let s = String::from("pwwkew");
        assert_eq!(Solution::length_of_longest_substring(s), 3)
    }

    #[test]
    fn test_group_4() {
        let s = String::from("");
        assert_eq!(Solution::length_of_longest_substring(s), 0)
    }

    #[test]
    fn test_group_5() {
        let s = String::from("b");
        assert_eq!(Solution::length_of_longest_substring(s), 1)
    }

    #[test]
    fn test_group_6() {
        let s = String::from("ba");
        assert_eq!(Solution::length_of_longest_substring(s), 2)
    }
}
