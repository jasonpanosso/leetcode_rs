pub struct Solution;

use crate::data_structures::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::delete_node_impl(&root, key)
    }

    fn delete_node_impl(
        node: &Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = node {
            let val = node.borrow().val;
            match val.cmp(&key) {
                Ordering::Greater => {
                    let left = &node.borrow().left.clone();
                    node.borrow_mut().left = Solution::delete_node_impl(left, key)
                }
                Ordering::Less => {
                    let right = &node.borrow().right.clone();
                    node.borrow_mut().right = Solution::delete_node_impl(right, key);
                }
                Ordering::Equal => {
                    let left = &node.borrow().left.clone();
                    if left.is_none() {
                        return node.borrow().right.clone();
                    }

                    let right = &node.borrow().right.clone();
                    if right.is_none() {
                        return node.borrow().left.clone();
                    }

                    let min_value_right_subtree = Self::find_min_node_value(right).unwrap();

                    node.borrow_mut().val = Self::find_min_node_value(right).unwrap();
                    node.borrow_mut().right =
                        Self::delete_node_impl(right, min_value_right_subtree);
                }
            }
        }
        node.clone()
    }

    fn find_min_node_value(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = node {
            if node.borrow().left.is_some() {
                Self::find_min_node_value(&node.borrow().left)
            } else {
                Some(node.borrow().val)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)];
        let root = TreeNode::from_vec(tree);

        let expected_tree = vec![Some(5), Some(4), Some(6), Some(2), None, None, Some(7)];
        let expected_root = TreeNode::from_vec(expected_tree);

        assert_eq!(Solution::delete_node(root, 3), expected_root);
    }
}
