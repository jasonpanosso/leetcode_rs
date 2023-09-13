pub struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let senate = String::from("RD");
        let expected = String::from("Radiant");

        assert_eq!(Solution::predict_party_victory(senate), expected)
    }

    #[test]
    fn test_example_2() {
        let senate = String::from("RDD");
        let expected = String::from("Dire");

        assert_eq!(Solution::predict_party_victory(senate), expected)
    }
}
