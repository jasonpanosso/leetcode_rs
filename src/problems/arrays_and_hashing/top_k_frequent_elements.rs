pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::{BinaryHeap, HashMap};

        let mut hash_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for num in nums {
            *hash_map.entry(num).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<(i32, i32)> =
            hash_map.drain().map(|(num, count)| (count, num)).collect();

        let mut output = Vec::with_capacity(k as usize);
        for _ in 0..k {
            if let Some((_count, num)) = heap.pop() {
                output.push(num);
            } else {
                break
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_vec() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        assert_eq!(Solution::top_k_frequent(nums, k), vec![1, 2]);
    }

    #[test]
    fn test_1_element_vec() {
        let nums = vec![1];
        let k = 1;
        assert_eq!(Solution::top_k_frequent(nums, k), vec![1]);
    }
}
