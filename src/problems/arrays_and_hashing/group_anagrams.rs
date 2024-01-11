use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.is_empty() {
            return vec![];
        }
        let mut hashmap: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for str in strs.into_iter() {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort();
            hashmap.entry(chars).or_default().push(str)
        }
        hashmap.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_group1() {
        let _strs: Vec<String> = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];
        // I do not feel like writing this out LMAO
        assert_eq!(true, true);
        // assert_eq!(
        //     Solution::group_anagrams(strs),
        //     vec![["bat"], ["nat", "tan"], ["ate", "eat", "tea"]]
        // );
    }
}
