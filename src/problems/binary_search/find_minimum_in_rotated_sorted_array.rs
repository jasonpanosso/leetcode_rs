pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut pivot = 0;

        while left <= right {
            pivot = left + (right - left) / 2;
            if nums[pivot] < nums[right] {
                right = pivot
            } else {
                left = pivot + 1
            }
        }
        nums[pivot]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let nums = vec![3, 4, 5, 1, 2];
        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn test_search_2() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        assert_eq!(Solution::find_min(nums), 0);
    }

    #[test]
    fn test_search_unrotated() {
        let nums = vec![11, 13, 15, 17];
        assert_eq!(Solution::find_min(nums), 11);
    }

    #[test]
    fn test_len_nums_1() {
        let nums = vec![1];
        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn test_len_nums_2() {
        let nums = vec![2, 1];
        assert_eq!(Solution::find_min(nums), 1);
    }

    #[test]
    fn test_len_nums_2_unrotated() {
        let nums = vec![1, 2];
        assert_eq!(Solution::find_min(nums), 1);
    }
}
