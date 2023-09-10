pub struct Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0 as usize, nums.len() - 1);
        let mut pivot = 0;

        while left <= right {
            pivot = left + (right - left) / 2;
            if nums[pivot] < nums[right] {
                right = pivot;
            } else {
                left = pivot + 1;
            }
        }

        left = 0;
        right = nums.len();
        let unrotated = [&nums[pivot..], &nums[..pivot]].concat();

        while left < right {
            let mid = left + (right - left) / 2;
            match target.cmp(&unrotated[mid]) {
                Ordering::Equal => return ((mid + pivot) % nums.len()) as i32,
                Ordering::Greater => left = mid + 1,
                Ordering::Less => right = mid,
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_target_exists() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(Solution::search(nums, target), 4);
    }

    #[test]
    fn test_search_target_dne() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_single_element_vec() {
        let nums = vec![0];
        let target = 0;
        assert_eq!(Solution::search(nums, target), 0);
    }

    #[test]
    fn test_single_element_vec_target_dne() {
        let nums = vec![0];
        let target = 1;
        assert_eq!(Solution::search(nums, target), -1);
    }

    #[test]
    fn test_unrotated_nums() {
        let nums = vec![1, 3];
        let target = 3;
        assert_eq!(Solution::search(nums, target), 1);
    }

    #[test]
    fn test_strange() {
        let nums = vec![3, 5, 1];
        let target = 3;
        assert_eq!(Solution::search(nums, target), 0);
    }

    #[test]
    fn test_strange_2() {
        let nums = vec![3, 1];
        let target = 3;
        assert_eq!(Solution::search(nums, target), 0);
    }
}
