pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut output = 0;
        let mut left_index = 0;
        let mut right_index = height.len() - 1;

        while left_index < right_index {
            output = output.max(
                height[left_index].min(height[right_index]) * (right_index - left_index) as i32,
            );

            match height[left_index] > height[right_index] {
                true => right_index -= 1,
                false => left_index += 1,
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);
    }

    #[test]
    fn test_group_2() {
        let height = vec![1, 1];
        assert_eq!(Solution::max_area(height), 1);
    }
}
