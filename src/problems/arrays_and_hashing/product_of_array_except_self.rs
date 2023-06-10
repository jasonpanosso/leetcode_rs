pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len();
        let mut output = vec![1; size];

        let mut left_prod = 1;
        let mut right_prod = 1;

        let mut j = size;

        for i in 0..size {
            j -= 1;
            output[i] *= left_prod;
            output[j] *= right_prod;

            right_prod *= nums[j];
            left_prod *= nums[i];
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_positive_nums() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(nums), expected)
    }

    #[test]
    fn test_with_zero() {
        let nums = vec![1, 2, 0, 4];
        let expected = vec![0, 0, 8, 0];
        assert_eq!(Solution::product_except_self(nums), expected)
    }

    #[test]
    fn test_with_negatives() {
        let nums = vec![1, -2, 3, -4];
        let expected = vec![24, -12, 8, -6];
        assert_eq!(Solution::product_except_self(nums), expected)
    }
}
