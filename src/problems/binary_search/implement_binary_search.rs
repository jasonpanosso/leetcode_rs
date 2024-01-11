use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            match target.cmp(&nums[mid]) {
                Ordering::Equal => return mid as i32,
                Ordering::Greater => left = mid + 1,
                Ordering::Less => right = mid,
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_target_exists() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        assert_eq!(Solution::search(nums, target), 4);
    }

    #[test]
    fn test_search_target_dne() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_vec_len_1() {
        let nums = vec![5];
        let target = 5;
        assert_eq!(Solution::search(nums, target), 0);
    }
}
