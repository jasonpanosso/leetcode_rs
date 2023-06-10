use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut output = 0;

        for num in set.iter() {
            if set.contains(&(num - 1)) {
                continue;
            }
            let mut current_num = num + 1;
            let mut count = 1;

            while set.contains(&current_num) {
                count += 1;
                current_num += 1;
            }
            output = output.max(count);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sequence() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4)
    }

    #[test]
    fn test_sequence_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9)
    }
}
