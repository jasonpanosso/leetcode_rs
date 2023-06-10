use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut hashmap: HashMap<char, i32> = HashMap::new();

        for (i, j) in s.chars().zip(t.chars()) {
            hashmap
                .entry(i)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            hashmap
                .entry(j)
                .and_modify(|count| *count -= 1)
                .or_insert(-1);
        }
        for count in hashmap.values() {
            if count != &0i32 {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram() {
        let s = String::from("rat");
        let t = String::from("tar");
        assert_eq!(Solution::is_anagram(s, t), true);
    }

    #[test]
    fn test_non_anagram() {
        let s = String::from("one");
        let t = String::from("two");
        assert_ne!(Solution::is_anagram(s, t), true);
    }

    #[test]
    fn test_differing_length() {
        let s = String::from("thre");
        let t = String::from("three");
        assert_ne!(Solution::is_anagram(s, t), true);
    }
}
