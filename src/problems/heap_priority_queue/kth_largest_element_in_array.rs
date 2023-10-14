pub struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from(nums);

        for _ in 0..k - 1 {
            heap.pop();
        }

        heap.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 2, 1, 5, 7, 4];
        let k = 2;

        let expected = 5;

        assert_eq!(Solution::find_kth_largest(nums, k), expected);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![3, 2, 3, 1, 2, 4, 5, 5, 6];
        let k = 4;

        let expected = 4;

        assert_eq!(Solution::find_kth_largest(nums, k), expected);
    }
}
