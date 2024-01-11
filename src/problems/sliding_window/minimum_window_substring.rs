pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut t_count_arr = [0; 52];
        let mut s_count_arr = [0; 52];

        for ch in t.chars() {
            t_count_arr[Self::get_index(ch)] += 1;
        }

        let s_chars: Vec<char> = s.chars().collect();
        let mut min_len = usize::MAX;
        let mut min_start = 0;
        let mut needed_chars = t.len();
        let mut left: usize = 0;

        for right in 0..s_chars.len() {
            let right_index = Self::get_index(s_chars[right]);
            s_count_arr[right_index] += 1;

            if t_count_arr[right_index] > 0 && s_count_arr[right_index] <= t_count_arr[right_index]
            {
                needed_chars -= 1;
            }

            while needed_chars == 0 {
                if right - left + 1 < min_len {
                    min_len = right - left + 1;
                    min_start = left;
                }

                let left_index = Self::get_index(s_chars[left]);
                s_count_arr[left_index] -= 1;

                if t_count_arr[left_index] > 0 && s_count_arr[left_index] < t_count_arr[left_index]
                {
                    needed_chars += 1;
                }
                left += 1;
            }
        }

        if min_len == usize::MAX {
            String::from("")
        } else {
            s_chars[min_start..min_start + min_len].iter().collect()
        }
    }

    fn get_index(ch: char) -> usize {
        if ch.is_uppercase() {
            ch as usize - 'A' as usize
        } else {
            26 + ch as usize - 'a' as usize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_standard_input_expect_true() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABC");
        assert_eq!(Solution::min_window(s, t), String::from("BANC"))
    }

    #[test]
    fn test_single_char_expect_true() {
        let s = String::from("a");
        let t = String::from("a");
        assert_eq!(Solution::min_window(s, t), String::from("a"))
    }

    #[test]
    fn test_invalid_input_t_longer_than_a_expect_empty_string() {
        let s = String::from("a");
        let t = String::from("aa");
        assert_eq!(Solution::min_window(s, t), String::from(""))
    }

    #[test]
    fn test_valid_input_aaabc() {
        let s = String::from("aaabc");
        let t = String::from("abc");
        assert_eq!(Solution::min_window(s, t), String::from("abc"))
    }

    #[test]
    fn test_differing_cases() {
        let s = String::from("ab");
        let t = String::from("A");
        assert_eq!(Solution::min_window(s, t), String::from(""))
    }
}
