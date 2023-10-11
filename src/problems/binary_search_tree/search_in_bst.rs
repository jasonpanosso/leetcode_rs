pub struct Solution;

use crate::data_structures::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            return match node.borrow().val.cmp(&val) {
                Ordering::Equal => Some(node.clone()),
                Ordering::Greater => Self::search_bst(node.borrow().left.clone(), val),
                Ordering::Less => Self::search_bst(node.borrow().right.clone(), val),
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![Some(4), Some(2), Some(7), Some(1), Some(3)];

        let root = TreeNode::from_vec(tree);

        let expected_tree = vec![Some(2), Some(1), Some(3)];
        let expected_root = TreeNode::from_vec(expected_tree);

        assert_eq!(Solution::search_bst(root, 2), expected_root);
    }
}
