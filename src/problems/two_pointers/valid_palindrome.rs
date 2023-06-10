pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .chars()
            .map(|char| char.to_ascii_lowercase())
            .filter(|char| char.is_ascii_alphanumeric())
            .collect();
        let len = chars.len();
        for i in 0..len / 2 {
            if chars[i] != chars[len - 1 - i] {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_palindrome() {
        let s = String::from("A man, a plan, a canal: Panama");
        assert!(Solution::is_palindrome(s));
    }

    #[test]
    fn test_invalid_palindrome() {
        let s = String::from("race a car");
        assert!(!Solution::is_palindrome(s));
    }

    #[test]
    fn test_string_only_spaces() {
        let s = String::from(" ");
        assert!(Solution::is_palindrome(s));
    }
}
