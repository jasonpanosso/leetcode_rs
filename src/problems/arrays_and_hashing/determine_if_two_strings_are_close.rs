pub struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let map1 = word1
            .chars()
            .fold(HashMap::with_capacity(word1.len()), |mut map, char| {
                map.entry(char).and_modify(|count| *count += 1).or_insert(1);
                map
            });

        let map2 = word2
            .chars()
            .fold(HashMap::with_capacity(word2.len()), |mut map, char| {
                map.entry(char).and_modify(|count| *count += 1).or_insert(1);
                map
            });

        if map1.keys().collect::<HashSet<_>>() != map2.keys().collect::<HashSet<_>>() {
            return false;
        }

        let counts1 = map1
            .values()
            .fold(HashMap::with_capacity(word1.len()), |mut map, val| {
                map.entry(val).and_modify(|count| *count += 1).or_insert(1);
                map
            });

        let counts2 = map2
            .values()
            .fold(HashMap::with_capacity(word2.len()), |mut map, val| {
                map.entry(val).and_modify(|count| *count += 1).or_insert(1);
                map
            });

        counts1 == counts2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let word1 = String::from("abc");
        let word2 = String::from("bca");
        let expected = true;

        assert_eq!(Solution::close_strings(word1, word2), expected);
    }

    #[test]
    fn test_example_2() {
        let word1 = String::from("a");
        let word2 = String::from("aa");
        let expected = false;

        assert_eq!(Solution::close_strings(word1, word2), expected);
    }

    #[test]
    fn test_example_3() {
        let word1 = String::from("abbzzca");
        let word2 = String::from("babzzcz");
        let expected = false;

        assert_eq!(Solution::close_strings(word1, word2), expected);
    }

    #[test]
    fn test_example_4() {
        let word1 = String::from("aaabbbbccddeeeeefffff");
        let word2 = String::from("aaaaabbcccdddeeeeffff");
        let expected = false;

        assert_eq!(Solution::close_strings(word1, word2), expected);
    }
}
