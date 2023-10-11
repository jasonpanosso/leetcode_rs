pub struct Solution;

use crate::data_structures::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut output: Vec<i32> = vec![];
        if root.is_none() {
            return output;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        while queue.len() > 0 {
            let queue_len = queue.len();
            for i in 0..queue_len {
                if let Some(node) = queue.pop_front() {
                    if i == queue_len - 1 {
                        output.push(node.borrow().val)
                    }

                    if let Some(left) = node.borrow().left.clone() {
                        queue.push_back(left);
                    }

                    if let Some(right) = node.borrow().right.clone() {
                        queue.push_back(right);
                    }
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![Some(1), Some(2), Some(3), None, Some(5), None, Some(4)];

        let root = TreeNode::from_vec(tree);
        let expected = vec![1, 3, 4];

        assert_eq!(Solution::right_side_view(root), expected);
    }

    #[test]
    fn test_example_2() {
        let tree = vec![Some(1), None, Some(3)];

        let root = TreeNode::from_vec(tree);
        let expected = vec![1, 3];

        assert_eq!(Solution::right_side_view(root), expected);
    }

    #[test]
    fn test_example_3() {
        let tree = vec![Some(1), Some(2)];

        let root = TreeNode::from_vec(tree);
        let expected = vec![1, 2];

        assert_eq!(Solution::right_side_view(root), expected);
    }
}
