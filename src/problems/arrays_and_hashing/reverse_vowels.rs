pub struct Solution;

impl Solution {
    fn is_vowel(c: char) -> bool {
        matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
    }

    pub fn reverse_vowels(s: String) -> String {
        let mut output: Vec<char> = s.chars().collect();
        let mut i = 0;
        let mut j = s.len() - 1;

        while i < j {
            match (Self::is_vowel(output[i]), Self::is_vowel(output[j])) {
                (true, true) => {
                    output.swap(i, j);
                    i += 1;
                    j -= 1;
                }
                (true, false) => j -= 1,
                (false, true) => i += 1,
                (false, false) => {
                    i += 1;
                    j -= 1;
                }
            }
        }
        output.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("hello");
        let expected = String::from("holle");

        assert_eq!(Solution::reverse_vowels(s), expected);
    }

    #[test]
    fn test_example_2() {
        let s = String::from("leetcode");
        let expected = String::from("leotcede");

        assert_eq!(Solution::reverse_vowels(s), expected);
    }
}
