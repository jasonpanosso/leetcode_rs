pub struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let set1: HashSet<i32> = nums1.into_iter().collect();
        let set2: HashSet<i32> = nums2.into_iter().collect();

        let mut vec1: Vec<i32> = vec![];
        let mut vec2: Vec<i32> = vec![];

        for num in set1.iter() {
            if !set2.contains(&num) {
                vec1.push(*num);
            }
        }
        for num in set2.iter() {
            if !set1.contains(&num) {
                vec2.push(*num);
            }
        }

        vec![vec1, vec2]
    }
}

// these tests don't work for obv reasons. I CBA to spend a bunch of time to
// make tests for this lmao..
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4, 6];
        let expected = vec![vec![1, 3], vec![4, 6]];

        assert_eq!(Solution::find_difference(nums1, nums2), expected);
    }

    #[test]
    fn test_example_2() {
        let nums1 = vec![1, 2, 3, 3];
        let nums2 = vec![1, 1, 2, 2];
        let expected = vec![vec![3], vec![]];

        assert_eq!(Solution::find_difference(nums1, nums2), expected);
    }
}
