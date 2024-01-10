use crate::data_structures::tree_node::TreeNode;
use std::{cell::RefCell, rc::Rc};

pub struct Solution {}

type TreeNodeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn is_subtree(root: Option<TreeNodeRef>, sub_root: Option<TreeNodeRef>) -> bool {
        if sub_root.is_none() {
            return true;
        }

        Self::dfs(&root, &sub_root)
    }

    fn dfs(maybe_node: &Option<TreeNodeRef>, sub_root: &Option<TreeNodeRef>) -> bool {
        if Self::is_same_tree(maybe_node, sub_root) {
            true
        } else if let Some(node) = maybe_node {
            let node = node.borrow();
            Self::dfs(&node.left, sub_root) || Self::dfs(&node.right, sub_root)
        } else {
            false
        }
    }

    fn is_same_tree(
        maybe_node: &Option<TreeNodeRef>,
        maybe_sub_node: &Option<TreeNodeRef>,
    ) -> bool {
        match (maybe_node, maybe_sub_node) {
            (Some(node), Some(sub_node)) => {
                let node = node.borrow();
                let sub_node = sub_node.borrow();
                node.val == sub_node.val
                    && Self::is_same_tree(&node.left, &sub_node.left)
                    && Self::is_same_tree(&node.right, &sub_node.right)
            }
            (None, None) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let root_nodes = vec![Some(3), Some(4), Some(5), Some(1), Some(2)];
        let root = TreeNode::from_vec(root_nodes);

        let subroot_nodes = vec![Some(4), Some(1), Some(2)];
        let subroot = TreeNode::from_vec(subroot_nodes);

        println!("root node: {:?}", root);
        println!("subroot node: {:?}", subroot);

        let expected = true;
        assert_eq!(Solution::is_subtree(root, subroot), expected);
    }

    #[test]
    fn test_example_3() {
        let root_nodes = vec![
            Some(3),
            Some(4),
            Some(5),
            Some(4),
            Some(5),
            None,
            None,
            Some(4),
            None,
            None,
            None,
            Some(1),
            Some(2),
        ];
        let root = TreeNode::from_vec(root_nodes);

        let subroot_nodes = vec![Some(4), Some(1), Some(2)];
        let subroot = TreeNode::from_vec(subroot_nodes);

        println!("root node: {:?}", root);
        println!("subroot node: {:?}", subroot);

        let expected = true;
        assert_eq!(Solution::is_subtree(root, subroot), expected);
    }
}
