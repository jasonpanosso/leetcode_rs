pub struct Solution;

use crate::data_structures::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        current_sum: i32,
        prefix_sums: &mut HashMap<i32, i32>,
    ) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();
            let new_sum = current_sum + node.val;

            let prev_sums = *prefix_sums.get(&(new_sum - target_sum)).unwrap_or(&0);
            *prefix_sums.entry(new_sum).or_insert(0) += 1;

            let total_sum = prev_sums
                + Self::dfs(&node.left, target_sum, new_sum, prefix_sums)
                + Self::dfs(&node.right, target_sum, new_sum, prefix_sums);

            *prefix_sums.entry(new_sum).or_insert(0) -= 1;

            total_sum
        } else {
            0
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut prefix_sums = HashMap::new();
        prefix_sums.insert(0, 1);
        Self::dfs(&root, target_sum, 0, &mut prefix_sums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![
            Some(10),
            Some(5),
            Some(-3),
            Some(3),
            Some(2),
            None,
            Some(11),
            Some(3),
            Some(-2),
            None,
            Some(1),
        ];

        let root = TreeNode::from_vec(tree);
        let target_sum = 8;
        let expected = 3;

        assert_eq!(Solution::path_sum(root, target_sum), expected);
    }

    #[test]
    fn test_example_2() {
        let tree = vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            Some(5),
            Some(1),
        ];

        let root = TreeNode::from_vec(tree);
        let target_sum = 22;
        let expected = 3;

        assert_eq!(Solution::path_sum(root, target_sum), expected);
    }

    #[test]
    fn test_example_3() {
        let tree = vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
        ];

        let root = TreeNode::from_vec(tree);
        let target_sum = 3;
        let expected = 2;

        assert_eq!(Solution::path_sum(root, target_sum), expected);
    }
}
