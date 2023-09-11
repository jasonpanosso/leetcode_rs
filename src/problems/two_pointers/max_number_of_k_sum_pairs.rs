use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut num_counts = HashMap::with_capacity(nums.len());
        let mut operation_count = 0;

        for num in nums.iter() {
            match num_counts.get_mut(&(k - num)) {
                Some(count) if *count > 0 => {
                    *count -= 1;
                    operation_count += 1
                }
                _ => {
                    num_counts
                        .entry(num)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }

        operation_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 4];
        let k = 5;
        let expected = 2;

        assert_eq!(Solution::max_operations(nums, k), expected);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![3, 1, 3, 4, 3];
        let k = 6;
        let expected = 1;

        assert_eq!(Solution::max_operations(nums, k), expected);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2];
        let k = 3;
        let expected = 4;

        assert_eq!(Solution::max_operations(nums, k), expected);
    }
}
