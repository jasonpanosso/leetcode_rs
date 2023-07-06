pub struct Solution;

// this is janky.. two binary searches is certainly not.. ideal
// looking through others solutions, I've seen some people just concating
// matrix into one vec(lmao), and some where people are doing some weird
// use of quotient/math trickery to discern which subvec the value is contained
// in. 100% can be optimized/more idiomatic, however it does run good XD

use std::cmp::Ordering;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (mut left, mut right) = (0, matrix.len());

        while left < right {
            let mid = left + (right - left) / 2;
            let len = matrix[mid].len();

            if target >= matrix[mid][0] {
                if target <= matrix[mid][len - 1] {
                    return Self::search(&matrix[mid], target);
                } else {
                    left = mid + 1;
                }
            } else {
                right = mid;
            }
        }
        false
    }

    fn search(nums: &Vec<i32>, target: i32) -> bool {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            match target.cmp(&nums[mid]) {
                Ordering::Equal => return true,
                Ordering::Greater => left = mid + 1,
                Ordering::Less => right = mid,
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_target_exists() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        assert!(Solution::search_matrix(matrix, target));
    }

    #[test]
    fn test_search_target_dne() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 13;
        assert!(!Solution::search_matrix(matrix, target));
    }
}
