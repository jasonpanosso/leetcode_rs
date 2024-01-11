pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let longer_word: &str;
        let shorter_word: &str;
        if word1.len() >= word2.len() {
            longer_word = &word1;
            shorter_word = &word2;
        } else {
            longer_word = &word2;
            shorter_word = &word1;
        }

        word1
            .chars()
            .zip(word2.chars())
            .fold(String::from(""), |acc, cur| {
                acc + &cur.0.to_string() + &cur.1.to_string()
            })
            + &longer_word[shorter_word.len()..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard() {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");

        let expected = String::from("apbqcr");

        assert_eq!(Solution::merge_alternately(word1, word2), expected);
    }

    #[test]
    fn test_word2_longer() {
        let word1 = String::from("ab");
        let word2 = String::from("pqrs");

        let expected = String::from("apbqrs");

        assert_eq!(Solution::merge_alternately(word1, word2), expected);
    }

    #[test]
    fn test_word1_longer() {
        let word1 = String::from("abcd");
        let word2 = String::from("pq");

        let expected = String::from("apbqcd");

        assert_eq!(Solution::merge_alternately(word1, word2), expected);
    }

    #[test]
    fn test_word2_len_1() {
        let word1 = String::from("abcd");
        let word2 = String::from("p");

        let expected = String::from("apbcd");

        assert_eq!(Solution::merge_alternately(word1, word2), expected);
    }

    #[test]
    fn test_word1_len_1() {
        let word1 = String::from("a");
        let word2 = String::from("pqrs");

        let expected = String::from("apqrs");

        assert_eq!(Solution::merge_alternately(word1, word2), expected);
    }
}
