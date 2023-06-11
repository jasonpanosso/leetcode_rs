pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut freq_map: HashMap<char, i32> = HashMap::with_capacity(s.len());
        let mut max_freq = 0;

        let mut window_size = 0;
        let mut left_index = 0;

        let s_chars: Vec<char> = s.chars().collect();

        for &ch in s_chars.iter() {
            let count = freq_map
                .entry(ch)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            max_freq = max_freq.max(*count);
            window_size += 1;

            if window_size - max_freq > k {
                let char_out = s_chars[left_index];
                let count = freq_map
                    .entry(char_out)
                    .and_modify(|count| *count -= 1)
                    .or_insert(0);
                if *count == 0 {
                    freq_map.remove(&char_out);
                }
                left_index += 1;
                window_size -= 1;
            }
        }
        window_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abab_string_with_two_replacements() {
        let string = String::from("ABAB");
        let k = 2;
        assert_eq!(Solution::character_replacement(string, k), 4)
    }

    #[test]
    fn test_aababba_string_with_one_replacement() {
        let string = String::from("AABABBA");
        let k = 1;
        assert_eq!(Solution::character_replacement(string, k), 4)
    }

    #[test]
    fn test_all_same_characters_no_replacements_needed() {
        let string = String::from("AAAAA");
        let k = 3;
        assert_eq!(Solution::character_replacement(string, k), 5)
    }

    #[test]
    fn test_no_replacements_k_0() {
        let string = String::from("ABABAA");
        let k = 0;
        assert_eq!(Solution::character_replacement(string, k), 2)
    }

    #[test]
    fn test_empty_string_input() {
        let string = String::from("");
        let k = 3;
        assert_eq!(Solution::character_replacement(string, k), 0)
    }

    // #[test]
    // fn test() {
    //     let string = String::from("");
    //     let k = ;
    //     assert!(Solution::character_replacement(string, k) == )
    // }
}
