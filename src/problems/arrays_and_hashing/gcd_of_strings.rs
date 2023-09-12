pub struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut str1_gcd = String::from("");
        let mut output = String::from("");

        let mut str_slice = &str1[0..str2.len().min(str1.len())];

        for i in 0..str_slice.len() {
            
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard() {
        let str1 = String::from("ABCABC");
        let str2 = String::from("ABC");

        let expected = String::from("ABC");

        assert_eq!(Solution::gcd_of_strings(str1, str2), expected);
    }

    #[test]
    fn test_unmatched_multiples() {
        let word1 = String::from("ABABAB");
        let word2 = String::from("ABAB");

        let expected = String::from("AB");

        assert_eq!(Solution::gcd_of_strings(word1, word2), expected);
    }

    #[test]
    fn test_no_gcd() {
        let word1 = String::from("LEET");
        let word2 = String::from("CODE");

        let expected = String::from("");

        assert_eq!(Solution::gcd_of_strings(word1, word2), expected);
    }
}
