pub struct Solution;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut hash_map: HashMap<i32, i32> = HashMap::with_capacity(arr.len());

        for num in arr {
            hash_map
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let values = hash_map.into_values();

        values.len() == values.collect::<HashSet<i32>>().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        let expected = true;

        assert_eq!(Solution::unique_occurrences(arr), expected);
    }

    #[test]
    fn test_example_2() {
        let arr = vec![1, 2];
        let expected = false;

        assert_eq!(Solution::unique_occurrences(arr), expected);
    }

    #[test]
    fn test_example_3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        let expected = true;

        assert_eq!(Solution::unique_occurrences(arr), expected);
    }
}
