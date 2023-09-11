pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            println!("{:?}", nums);
            println!("{:?}", nums[left]);
            if nums[left] == 0 {
                nums.remove(left);
                nums.push(0);
                right -= 1;
            } else {
                left += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let expected = vec![1, 3, 12, 0, 0];

        Solution::move_zeroes(&mut nums);

        println!("{:?} {:?}", nums, expected);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![0];
        let expected = vec![0];

        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_example_3() {
        let mut nums = vec![0, 0, 1];
        let expected = vec![1, 0, 0];

        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }
}
