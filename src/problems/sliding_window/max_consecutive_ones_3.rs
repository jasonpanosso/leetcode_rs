pub struct Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut zero_count = 0;

        while right < nums.len() {
            if nums[right] == 0 {
                zero_count += 1;
            }

            if k < zero_count {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }
            right += 1;
        }

        (right - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k = 2;
        let expected = 6;

        assert_eq!(Solution::longest_ones(nums, k), expected)
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
        let k = 3;
        let expected = 10;

        assert_eq!(Solution::longest_ones(nums, k), expected)
    }
}
