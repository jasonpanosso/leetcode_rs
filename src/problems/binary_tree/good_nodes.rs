pub struct Solution;

use crate::data_structures::tree_node::TreeNode;
use std::{cell::RefCell, rc::Rc};

impl Solution {
    fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        if let Some(current) = node {
            let cur_value = current.borrow().val;
            let good_node = if cur_value >= max { 1 } else { 0 };

            match (&current.borrow().left, &current.borrow().right) {
                (None, None) => good_node,
                (left, right) => {
                    let new_max = max.max(cur_value);
                    good_node + Self::traverse(left, new_max) + Self::traverse(right, new_max)
                }
            }
        } else {
            0
        }
    }

    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::traverse(&root, i32::MIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree1 = vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)];

        let root1 = TreeNode::from_vec(tree1);

        let expected = 4;
        assert_eq!(Solution::good_nodes(root1), expected);
    }

    #[test]
    fn test_example_2() {
        let tree = vec![Some(3), Some(3), None, Some(4), Some(2)];

        let root = TreeNode::from_vec(tree);
        let expected = 3;
        assert_eq!(Solution::good_nodes(root), expected);
    }

    #[test]
    fn test_example_3() {
        let tree = vec![
            Some(2),
            None,
            Some(4),
            Some(10),
            Some(8),
            None,
            None,
            Some(4),
        ];

        let root = TreeNode::from_vec(tree);
        let expected = 4;
        assert_eq!(Solution::good_nodes(root), expected);
    }
}
