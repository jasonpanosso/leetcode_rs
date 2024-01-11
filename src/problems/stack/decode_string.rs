pub struct Solution;

use core::str::Chars;
impl Solution {
    fn helper(chars: &mut Chars) -> String {
        let mut output: String = String::from("");

        while let Some(char) = chars.next() {
            if char == ']' {
                return output;
            } else if char.is_ascii_digit() {
                let mut digits = String::from(char);

                for maybe_digit in chars.by_ref() {
                    if maybe_digit.is_ascii_digit() {
                        digits.push(maybe_digit)
                    } else {
                        break;
                    }
                }
                output += &(Solution::helper(chars).repeat(digits.parse::<usize>().unwrap()));
            } else {
                output.push(char);
            }
        }
        output
    }

    pub fn decode_string(s: String) -> String {
        let mut chars = s.chars();

        Solution::helper(&mut chars)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = String::from("3[a]2[bc]");
        let expected = String::from("aaabcbc");

        assert_eq!(Solution::decode_string(s), expected)
    }

    #[test]
    fn test_example_2() {
        let s = String::from("3[a2[c]]");
        let expected = String::from("accaccacc");

        assert_eq!(Solution::decode_string(s), expected)
    }

    #[test]
    fn test_example_3() {
        let s = String::from("2[abc]3[cd]ef");
        let expected = String::from("abcabccdcdcdef");

        assert_eq!(Solution::decode_string(s), expected)
    }

    #[test]
    fn test_large_repeat() {
        let s = String::from("100[leetcode]");
        let expected = String::from("leetcode").repeat(100);

        assert_eq!(Solution::decode_string(s), expected)
    }
}
