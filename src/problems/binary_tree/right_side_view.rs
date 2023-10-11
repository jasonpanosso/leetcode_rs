pub struct Solution;

use crate::data_structures::tree_node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut output = vec![];
        if let Some(root) = root {
            let mut queue = VecDeque::new();
            queue.push_back(root);

            while !queue.is_empty() {
                let queue_len = queue.len();
                for i in 0..queue_len {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();
                    if i == queue_len - 1 {
                        output.push(node.val);
                    }

                    if let Some(left) = &node.left {
                        queue.push_back(left.clone());
                    }

                    if let Some(right) = &node.right {
                        queue.push_back(right.clone());
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
