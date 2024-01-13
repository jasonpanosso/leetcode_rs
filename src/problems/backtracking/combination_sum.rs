use std::cmp::Ordering;
pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut current = vec![];

        Self::backtrack(0, &candidates, &mut current, &mut result, target);
        result
    }

    fn backtrack(
        index: usize,
        nums: &[i32],
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        target: i32,
    ) {
        if index == nums.len() {
            return;
        }

        let sum: i32 = current.iter().sum();
        match sum.cmp(&target) {
            Ordering::Equal => {
                result.push(current.clone());
                return;
            }
            Ordering::Greater => {
                return;
            }
            _ => {}
        }

        current.push(nums[index]);
        Self::backtrack(index, nums, current, result, target);

        current.pop();
        Self::backtrack(index + 1, nums, current, result, target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;

        let expected = vec![vec![2, 2, 3], vec![7]];

        assert_eq!(Solution::combination_sum(candidates, target), expected)
    }

    #[test]
    fn test_example_2() {
        let candidates = vec![2, 3, 5];
        let target = 8;

        let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];

        assert_eq!(Solution::combination_sum(candidates, target), expected)
    }

    #[test]
    fn test_example_3() {
        let candidates = vec![2];
        let target = 1;

        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(Solution::combination_sum(candidates, target), expected)
    }
}
