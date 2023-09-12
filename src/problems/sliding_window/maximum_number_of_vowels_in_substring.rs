pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let s_chars: Vec<char> = s.chars().collect();
        let mut current_vowels = 0;

        for i in 0..k {
            if Self::is_vowel(s_chars[i as usize]) {
                current_vowels += 1;
            }
        }

        let mut max_vowels = current_vowels;
        let mut left: usize = 0;
        let mut right = k as usize;

        while right < s.len() {
            if Self::is_vowel(s_chars[left]) {
                current_vowels -= 1;
            }
            if Self::is_vowel(s_chars[right]) {
                current_vowels += 1;
            }

            max_vowels = max_vowels.max(current_vowels);

            left += 1;
            right += 1;
        }

        max_vowels
    }

    fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("abciiidef");
        let k = 3;
        let expected = 3;

        assert_eq!(Solution::max_vowels(s, k), expected)
    }

    #[test]
    fn test_example_2() {
        let s = String::from("aeiou");
        let k = 2;
        let expected = 2;

        assert_eq!(Solution::max_vowels(s, k), expected)
    }

    #[test]
    fn test_example_3() {
        let s = String::from("leetcode");
        let k = 3;
        let expected = 2;

        assert_eq!(Solution::max_vowels(s, k), expected)
    }
}
