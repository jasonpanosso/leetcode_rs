pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();
        let mut prefix_sum = 0;

        for (i, val) in nums.iter().enumerate() {
            if prefix_sum == total_sum - prefix_sum - val {
                return i as i32;
            }

            prefix_sum += val;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        let expected = 3;

        assert_eq!(Solution::pivot_index(nums), expected)
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 2, 3];
        let expected = -1;

        assert_eq!(Solution::pivot_index(nums), expected)
    }

    #[test]
    fn test_example_3() {
        let nums = vec![2, 1, -1];
        let expected = 0;

        assert_eq!(Solution::pivot_index(nums), expected)
    }
}
