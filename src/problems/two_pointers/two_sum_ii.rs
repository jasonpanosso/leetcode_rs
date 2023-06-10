use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn two_sum_ii(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut i, mut j) = (0, nums.len() - 1);

        while i < j {
            match (nums[i] + nums[j]).cmp(&target) {
                Ordering::Equal => return vec![(i as i32) + 1, (j as i32) + 1],
                Ordering::Greater => j -= 1,
                Ordering::Less => i += 1,
            }
        }
        vec![]
    }
}

// binary search
// impl Solution {
//     pub fn two_sum_ii(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         for (i, num) in nums.iter().enumerate() {
//             if let Some(j) = Self::binary_search(&nums[i + 1..], target - num) {
//                 let i = i as i32 + 1;
//                 let j = j as i32 + i + 1;
//                 return vec![i, j];
//             }
//         }
//         vec![]
//     }
//
//     fn binary_search(slice: &[i32], target: i32) -> Option<usize> {
//         let mut left = 0;
//         let mut right = slice.len();
//
//         while left < right {
//             let mid = left + (right - left) / 2;
//             match slice[mid].cmp(&target) {
//                 Ordering::Equal => return Some(mid),
//                 Ordering::Greater => right = mid,
//                 Ordering::Less => left = mid + 1,
//             }
//         }
//
//         None
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum_ii(nums, target), vec![1, 2])
    }
}
