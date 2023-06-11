pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() || s1.len() == 0 {
            return false;
        }
        let window_size = s1.len();

        let mut s1_freq_array = [0; 26];
        let mut s2_freq_array = [0; 26];
        for ch in s1.chars() {
            s1_freq_array[ch as usize - 'a' as usize] += 1;
        }

        let s2_chars: Vec<char> = s2.chars().collect();
        for (i, &ch) in s2_chars.iter().enumerate() {
            s2_freq_array[ch as usize - 'a' as usize] += 1;

            if i >= window_size {
                s2_freq_array[s2_chars[i - window_size] as usize - 'a' as usize] -= 1;
            }
            if s1_freq_array == s2_freq_array {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_reversed_permutation_expect_true() {
        let s1 = String::from("ab");
        let s2 = String::from("eidbaooo");

        assert!(Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn test_input_no_valid_permutation_expect_false() {
        let s1 = String::from("ab");
        let s2 = String::from("eidboaoo");

        assert!(!Solution::check_inclusion(s1, s2));
    }

    #[test]
    fn test_empty_s1() {
        let s1 = String::from("");
        let s2 = String::from("eidboaoo");

        assert!(!Solution::check_inclusion(s1, s2));
    }
}
