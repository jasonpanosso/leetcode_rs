pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let most_candies = candies.iter().max().unwrap();

        candies
            .iter()
            .map(|x| (x + extra_candies) >= *most_candies)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;

        let expected = vec![true, true, true, false, true];

        assert_eq!(
            Solution::kids_with_candies(candies, extra_candies),
            expected
        );
    }

    #[test]
    fn test_example_2() {
        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;

        let expected = vec![true, false, false, false, false];

        assert_eq!(
            Solution::kids_with_candies(candies, extra_candies),
            expected
        );
    }

    #[test]
    fn test_example_3() {
        let candies = vec![12, 1, 12];
        let extra_candies = 10;

        let expected = vec![true, false, true];

        assert_eq!(
            Solution::kids_with_candies(candies, extra_candies),
            expected
        );
    }
}
