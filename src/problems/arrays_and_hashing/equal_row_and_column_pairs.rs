pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let grid_len = grid.len();
        let mut pairs_count = 0;

        let map: HashMap<&Vec<i32>, i32> =
            grid.iter()
                .fold(HashMap::with_capacity(grid_len), |mut map, row| {
                    map.entry(row).and_modify(|count| *count += 1).or_insert(1);
                    map
                });

        let mut column: Vec<i32> = Vec::with_capacity(grid_len);

        for i in 0..grid_len {
            column.clear();

            (0..grid_len).for_each(|j| column.push(grid[j][i]));

            if let Some(count) = map.get(&column) {
                pairs_count += count;
            }
        }

        pairs_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]];
        let expected = 1;

        assert_eq!(Solution::equal_pairs(grid), expected)
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2],
        ];
        let expected = 3;

        assert_eq!(Solution::equal_pairs(grid), expected)
    }
}
