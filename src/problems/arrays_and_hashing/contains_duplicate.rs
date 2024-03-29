use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let set: HashSet<&i32> = nums.iter().collect();
        nums.len() != set.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_duplicates() {
        let nums = vec![1, 2, 3];
        assert!(!Solution::contains_duplicate(nums));
    }

    #[test]
    fn test_duplicates() {
        let nums = vec![1, 2, 3, 1];
        assert!(Solution::contains_duplicate(nums));
    }
}
