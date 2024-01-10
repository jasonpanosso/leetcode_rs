use crate::data_structures::tree_node::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution {}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<TreeNodeRef>) -> i32 {
        let mut max_diameter = 0;
        Self::dfs(&root, &mut max_diameter);

        max_diameter
    }

    fn dfs(maybe_node: &Option<TreeNodeRef>, max_diameter: &mut i32) -> i32 {
        if let Some(node) = maybe_node {
            let borrowed_node = node.borrow();

            let left_depth = Self::dfs(&borrowed_node.left, max_diameter);
            let right_depth = Self::dfs(&borrowed_node.right, max_diameter);

            *max_diameter = (*max_diameter).max(left_depth + right_depth);

            1 + left_depth.max(right_depth)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nodes = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
        let root = TreeNode::from_vec(nodes);

        println!("root node: {:?}", root);

        let expected = 3;
        assert_eq!(Solution::diameter_of_binary_tree(root), expected);
    }
}
