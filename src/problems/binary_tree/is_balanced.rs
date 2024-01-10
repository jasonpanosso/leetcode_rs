use crate::data_structures::tree_node::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution {}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::dfs(&root).1
    }

    fn dfs(maybe_node: &Option<TreeNodeRef>) -> (i32, bool) {
        if let Some(node) = maybe_node {
            let borrowed_node = node.borrow();

            let (left_depth, is_left_balanced) = Self::dfs(&borrowed_node.left);
            let (right_depth, is_right_balanced) = Self::dfs(&borrowed_node.right);

            if is_left_balanced && is_right_balanced && (left_depth - right_depth).abs() <= 1 {
                (1 + left_depth.max(right_depth), true)
            } else {
                (0, false)
            }
        } else {
            (0, true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nodes = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = TreeNode::from_vec(nodes);

        println!("root node: {:?}", root);

        let expected = true;
        assert_eq!(Solution::is_balanced(root), expected);
    }

    #[test]
    fn test_example_2() {
        let nodes = vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ];
        let root = TreeNode::from_vec(nodes);

        println!("root node: {:?}", root);

        let expected = false;
        assert_eq!(Solution::is_balanced(root), expected);
    }
}
