pub struct Solution;

// THIS SOLUTION IS CHEEKS. REVISIT!
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut output = 0;
        if height.len() <= 1 {
            return output;
        }

        let mut height = height;

        let (mut left, mut right) = (0, 1);
        while left < height.len() - 1 {
            if right == height.len() {
                height[left] = *height[left + 1..].iter().max().unwrap();
                right = left;
            } else if height[left] <= height[right] {
                for val in &height[left..right] {
                    output += height[left] - val
                }
                left = right;
            }
            right += 1;
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(height), 6);
    }

    #[test]
    fn test_group_2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height), 9);
    }

    #[test]
    fn test_group_3() {
        let height = vec![4, 2, 3];
        assert_eq!(Solution::trap(height), 1);
    }
}
