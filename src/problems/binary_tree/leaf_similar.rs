pub struct Solution;

use crate::data_structures::tree_node::TreeNode;
use std::{cell::RefCell, rc::Rc};

impl Solution {
    fn get_leaves(node: &Option<Rc<RefCell<TreeNode>>>, leaves: &mut Vec<i32>) {
        if let Some(current) = node {
            match (&current.borrow().left, &current.borrow().right) {
                (None, None) => leaves.push(current.borrow().val),
                (left, right) => {
                    Self::get_leaves(left, leaves);
                    Self::get_leaves(right, leaves);
                }
            }
        }
    }
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut leaves1: Vec<i32> = vec![];
        let mut leaves2: Vec<i32> = vec![];

        Self::get_leaves(&root1, &mut leaves1);
        Self::get_leaves(&root2, &mut leaves2);

        leaves1 == leaves2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree1 = vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(9),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ];

        let tree2 = vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(7),
            Some(4),
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(9),
            Some(8),
        ];

        let root1 = TreeNode::from_vec(tree1);
        let root2 = TreeNode::from_vec(tree2);
        // println!("{:?}", root1);
        // println!("{:?}", root2);
        assert!(Solution::leaf_similar(root1, root2));
    }

    #[test]
    fn test_example_2() {
        let tree1 = vec![Some(1), Some(2), Some(3)];
        let tree2 = vec![Some(1), Some(3), Some(2)];

        let root1 = TreeNode::from_vec(tree1);
        let root2 = TreeNode::from_vec(tree2);
        println!("{:?}", root1);
        println!("{:?}", root2);
        assert!(!Solution::leaf_similar(root1, root2));
    }
}
