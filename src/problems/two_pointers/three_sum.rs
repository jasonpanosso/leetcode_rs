pub struct Solution;

use std::cmp::Ordering;
use std::collections::HashSet;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        let len = nums.len();

        let mut nums = nums;
        nums.sort();
        for (i, num) in nums.iter().enumerate() {
            let mut left_index = i + 1;
            let mut right_index = len - 1;
            let target = 0 - num;

            while left_index < right_index {
                match (nums[left_index] + nums[right_index]).cmp(&target) {
                    Ordering::Equal => {
                        set.insert(vec![*num, nums[left_index], nums[right_index]]);
                        left_index += 1;
                    }
                    Ordering::Greater => right_index -= 1,
                    Ordering::Less => left_index += 1,
                }
            }
        }
        Vec::from_iter(set)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_1() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let answer = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let mut solution = Solution::three_sum(nums);
        solution.sort();
        assert_eq!(solution, answer);
    }

    #[test]
    fn test_group_2() {
        let nums = vec![0, 1, 1];
        let answer: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(nums), answer);
    }
}
