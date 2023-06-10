use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let difference = target - num;
            if let Some(&j) = hashmap.get(&difference) {
                return vec![j, i as i32];
            }
            hashmap.insert(num, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![1, 2, 3, 5];
        let target = 8i32;
        assert_eq!(Solution::two_sum(nums, target), vec![2, 3]);
    }

    #[test]
    fn test_two_sum_two() {
        let nums = vec![1, 1, 3, 5, 7, 2];
        let target = 5i32;
        assert_eq!(Solution::two_sum(nums, target), vec![2, 5]);
    }
}
