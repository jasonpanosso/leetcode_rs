pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut current = vec![];
        let mut result = vec![];

        Self::backtrack(&nums, 0, &mut current, &mut result);

        result
    }

    fn backtrack(nums: &[i32], index: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            result.push(current.clone());
            return;
        }

        current.push(nums[index]);
        Self::backtrack(nums, index + 1, current, result);

        current.pop();
        Self::backtrack(nums, index + 1, current, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3];

        let expected: Vec<Vec<i32>> = vec![
            vec![1, 2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1],
            vec![2, 3],
            vec![2],
            vec![3],
            vec![],
        ];

        assert_eq!(Solution::subsets(nums), expected)
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0];
        let expected: Vec<Vec<i32>> = vec![vec![0], vec![]];

        assert_eq!(Solution::subsets(nums), expected)
    }
}
