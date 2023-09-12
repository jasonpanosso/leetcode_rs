pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut zero_count = 0;

        // println!("===START===");
        // println!("NUMS INIT: {:?}", nums);
        while right < nums.len() {
            // println!("~~loop~~");
            //
            // println!("left: {left}");
            // println!("right: {right}");
            // println!("zero count: {zero_count}");
            // println!("longest: {longest_sequence}");
            // println!("nums start of loop: {:?}", &nums[left..right]);
            if nums[right] == 0 {
                zero_count += 1;
            }

            if zero_count > 1 {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }

            right += 1;
        }

        (right - left - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 1, 0, 1];
        let expected = 3;

        assert_eq!(Solution::longest_subarray(nums), expected)
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        let expected = 5;

        assert_eq!(Solution::longest_subarray(nums), expected)
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 1, 1];
        let expected = 2;

        assert_eq!(Solution::longest_subarray(nums), expected)
    }

    #[test]
    fn test_example_4() {
        let nums = vec![1, 1, 1, 1];
        let expected = 3;

        assert_eq!(Solution::longest_subarray(nums), expected)
    }

    #[test]
    fn test_example_5() {
        let nums = vec![1, 1, 1, 0];
        let expected = 3;

        assert_eq!(Solution::longest_subarray(nums), expected)
    }
}
